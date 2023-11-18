use anyhow::Context;
use egui_directx11::DirectX11Renderer;
use encryption_procmacro::encrypt;
use parking_lot::Mutex;
use retour::GenericDetour;
use std::sync::OnceLock;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HMODULE, HWND, TRUE},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_NULL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_11_1},
            Direct3D11::{
                D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_SAMPLE_DESC,
                },
                IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

use crate::{
    features, hook,
    render::dx11::definitions::{
        IDXGISWAPCHAIN_PRESENT_INDEX, IDXGISWAPCHAIN_RESIZE_BUFFERS_INDEX,
    },
    settings::SETTINGS,
    ui::{self},
    unhook, vfunction,
};

use self::definitions::{IDXGISwapChainPresent, IDXGISwapChainResizeBuffers};

use super::{fonts::FONTS, win32::INPUT};

mod definitions;

static SWAPCHAIN_RESIZE_BUFFERS: OnceLock<GenericDetour<IDXGISwapChainResizeBuffers>> =
    OnceLock::new();

static SWAPCHAIN_PRESENT: OnceLock<GenericDetour<IDXGISwapChainPresent>> = OnceLock::new();

static RENDERER: OnceLock<Mutex<DirectX11Renderer>> = OnceLock::new();

#[encrypt]
fn create_swapchain(window: HWND) -> anyhow::Result<IDXGISwapChain> {
    let flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    let feature_levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_10_0];

    let swapchain_description = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ..Default::default()
        },
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        OutputWindow: window,
        Windowed: TRUE,
        ..Default::default()
    };

    let mut device = None;
    let mut swapchain: Option<IDXGISwapChain> = None;

    unsafe {
        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_NULL,
            HMODULE::default(),
            flags,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&swapchain_description),
            Some(&mut swapchain),
            Some(&mut device),
            None,
            None,
        )
        .context("D3D11CreateDeviceAndSwapChain failed")?
    };

    swapchain.context("could not create d3d11 swapchain")
}

#[encrypt]
pub fn setup(window: HWND) -> anyhow::Result<()> {
    let swapchain = create_swapchain(window)?;

    let swapchain_present: IDXGISwapChainPresent =
        vfunction!(swapchain, IDXGISWAPCHAIN_PRESENT_INDEX);

    let swapchain_resize_buffers: IDXGISwapChainResizeBuffers =
        vfunction!(swapchain, IDXGISWAPCHAIN_RESIZE_BUFFERS_INDEX);

    hook!(SWAPCHAIN_PRESENT, swapchain_present, swapchain_present_hk)
        .context("could not hook SWAPCHAIN_PRESENT")?;

    hook!(
        SWAPCHAIN_RESIZE_BUFFERS,
        swapchain_resize_buffers,
        swapchain_resize_buffers_hk
    )
    .context("could not hook SWAPCHAIN_RESIZE_BUFFERS")?;

    Ok(())
}

#[encrypt]
pub fn destroy() -> anyhow::Result<()> {
    unhook!(SWAPCHAIN_PRESENT).context("could not unhook SWAPCHAIN_PRESENT")?;
    unhook!(SWAPCHAIN_RESIZE_BUFFERS).context("could not unhook SWAPCHAIN_RESIZE_BUFFERS")?;

    Ok(())
}

#[cfg_attr(not(debug_assertions), encrypt)]
fn swapchain_present_hk(swapchain: IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT {
    let hook = SWAPCHAIN_PRESENT
        .get()
        .expect(&"swapchain present hook is not initialized");

    let mut renderer = RENDERER
        .get_or_init(|| {
            Mutex::new(
                DirectX11Renderer::init_from_swapchain(&swapchain, egui::Context::default())
                    .expect(&"could not create dx11 renderer"),
            )
        })
        .lock();

    let input = INPUT
        .get()
        .expect(&"win32::INPUT is not initialized")
        .lock()
        .collect_input()
        .expect(&"could not collect input");

    let mut settings = SETTINGS.lock();

    if let Err(e) = renderer.paint(&swapchain, &mut settings, input, |ctx, settings| {
        let Some(fonts) = &*FONTS.lock() else {
            log::error!("fonts are not set up");
            return;
        };

        ctx.set_fonts(fonts.clone()); // retarded

        ctx.tessellation_options_mut(|options| {
            options.feathering = false;
        });

        features::visuals::esp::render(ctx.debug_painter(), &settings.visuals.esp);

        ui::render(ctx, settings);
    }) {
        log::error!("rendering error: {e}");
    }

    unsafe { hook.call(swapchain, sync_interval, flags) }
}

#[encrypt]
fn swapchain_resize_buffers_hk(
    swapchain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    swapchain_flags: u32,
) -> HRESULT {
    let hook = SWAPCHAIN_RESIZE_BUFFERS
        .get()
        .expect(&"could not get SWAPCHAIN_RESIZE_BUFFERS hook");

    let mut renderer = RENDERER
        .get()
        .expect(&"dx11 renderer is not initialized while resizing buffers")
        .lock();

    renderer
        .resize_buffers(&swapchain, || unsafe {
            hook.call(
                swapchain.clone(),
                buffer_count,
                width,
                height,
                new_format,
                swapchain_flags,
            )
        })
        .expect(&"could not resize buffers")
}
