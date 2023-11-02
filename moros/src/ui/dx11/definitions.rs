use windows::{
    core::HRESULT,
    Win32::Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISwapChain},
};

pub const IDXGISWAPCHAIN_PRESENT_INDEX: isize = 8;
pub type IDXGISwapChainPresent = unsafe fn(IDXGISwapChain, u32, u32) -> HRESULT;

pub const IDXGISWAPCHAIN_RESIZE_BUFFERS_INDEX: isize = 13;
pub type IDXGISwapChainResizeBuffers =
    unsafe fn(IDXGISwapChain, u32, u32, u32, DXGI_FORMAT, u32) -> HRESULT;
