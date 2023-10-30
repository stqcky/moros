use std::sync::{Arc, OnceLock};

use anyhow::Context;
use ash::vk::{self, PFN_vkAcquireNextImageKHR, PFN_vkAcquireNextImage2KHR, PFN_vkQueuePresentKHR, PFN_vkCreateSwapchainKHR};
use retour::GenericDetour;
use vulkano::{
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    VulkanLibrary,
};

use crate::{hook, unhook};

static DEVICE: OnceLock<vk::Device> = OnceLock::new();

fn create_device() -> anyhow::Result<Arc<Device>> {
    let library = VulkanLibrary::new().context("could not create vulkan library")?;

    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: InstanceExtensions {
                khr_surface: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .context("could not create vulkan instance")?;

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..Default::default()
    };

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .context("could not enumerate physical devices")?
        .filter(|device| device.supported_extensions().contains(&device_extensions))
        .filter_map(|device| {
            device
                .queue_family_properties()
                .iter()
                .enumerate()
                .position(|(_, queue)| queue.queue_flags.intersects(QueueFlags::GRAPHICS))
                .map(|queue_index| (device, queue_index as u32))
        })
        .min_by_key(|(device, _)| match device.properties().device_type {
            PhysicalDeviceType::IntegratedGpu => 0,
            PhysicalDeviceType::DiscreteGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .context("could not find suitable physical device")?;

    log::info!("using device: {}", physical_device.properties().device_name);

    let (device, _) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .context("could not create vulkan device")?;

    Ok(device)
}

pub fn setup() -> anyhow::Result<()> {
    let device = create_device()?;

    let khr_swapchain = &device.fns().khr_swapchain;

    let acquire_next_image = khr_swapchain.acquire_next_image_khr;
    let acquire_next_image2 = khr_swapchain.acquire_next_image2_khr;
    let queue_present = khr_swapchain.queue_present_khr;
    let create_swapchain = khr_swapchain.create_swapchain_khr;

    hook!(
        ACQUIRE_NEXT_IMAGE,
        acquire_next_image,
        acquire_next_image_hk
    );

    hook!(
        ACQUIRE_NEXT_IMAGE2,
        acquire_next_image2,
        acquire_next_image2_hk
    );

    hook!(QUEUE_PRESENT, queue_present, queue_present_hk);
    hook!(CREATE_SWAPCHAIN, create_swapchain, create_swapchain_hk);

    Ok(())
}

pub fn unhook() -> anyhow::Result<()> {
    unhook!(ACQUIRE_NEXT_IMAGE);
    unhook!(ACQUIRE_NEXT_IMAGE2);
    unhook!(QUEUE_PRESENT);
    unhook!(CREATE_SWAPCHAIN);

    Ok(())
}

static ACQUIRE_NEXT_IMAGE: OnceLock<GenericDetour<PFN_vkAcquireNextImageKHR>> = OnceLock::new();
extern "system" fn acquire_next_image_hk(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    timeout: u64,
    semaphore: vk::Semaphore,
    fence: vk::Fence,
    image_index: *mut u32,
) -> vk::Result {
    let _ = DEVICE.set(device);

    let Some(hook) = ACQUIRE_NEXT_IMAGE.get() else {
        return vk::Result::ERROR_UNKNOWN;
    };

    unsafe { hook.call(device, swapchain, timeout, semaphore, fence, image_index) }
}

static ACQUIRE_NEXT_IMAGE2: OnceLock<GenericDetour<PFN_vkAcquireNextImage2KHR>> = OnceLock::new();
extern "system" fn acquire_next_image2_hk(
    device: vk::Device,
    acquire_info: *const vk::AcquireNextImageInfoKHR,
    image_index: *mut u32,
) -> vk::Result {
    let _ = DEVICE.set(device);

    let Some(hook) = ACQUIRE_NEXT_IMAGE2.get() else {
        log::error!("could not get vkAcquireNextImage2KHR hook");
        return vk::Result::ERROR_UNKNOWN;
    };

    unsafe { hook.call(device, acquire_info, image_index) }
}

static QUEUE_PRESENT: OnceLock<GenericDetour<PFN_vkQueuePresentKHR>> = OnceLock::new();
extern "system" fn queue_present_hk(
    queue: vk::Queue,
    present_info: *const vk::PresentInfoKHR,
) -> vk::Result {
    log::info!("queue present. queue: {:?}", queue);

    let Some(hook) = QUEUE_PRESENT.get() else {
        log::error!("could not get vkQueuePresentKHR hook");
        return vk::Result::ERROR_UNKNOWN;
    };

    unsafe { hook.call(queue, present_info) }
}

static CREATE_SWAPCHAIN: OnceLock<GenericDetour<PFN_vkCreateSwapchainKHR>> = OnceLock::new();
extern "system" fn create_swapchain_hk(
    device: vk::Device,
    create_info: *const vk::SwapchainCreateInfoKHR,
    allocator: *const vk::AllocationCallbacks,
    swapchain: *mut vk::SwapchainKHR,
) -> vk::Result {
    log::info!("create swapchain. device: {:?}", device);

    let Some(hook) = CREATE_SWAPCHAIN.get() else {
        log::error!("could not get vkCreateSwapchainKHR hook");
        return vk::Result::ERROR_UNKNOWN;
    };

    unsafe { hook.call(device, create_info, allocator, swapchain) }
}

fn render(queue: vk::Queue, present_info: *const vk::PresentInfoKHR) {
    let Some(device) = DEVICE.get() else {
        return;
    };
}
