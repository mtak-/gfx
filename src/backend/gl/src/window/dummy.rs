use crate::{Backend, Device, PhysicalDevice, QueueFamily, native};

use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct Surface {
    pub(crate) swapchain: Option<Swapchain>,
}

impl hal::Surface<Backend> for Surface {
    fn compatibility(
        &self,
        _: &PhysicalDevice,
    ) -> (
        hal::SurfaceCapabilities,
        Option<Vec<hal::format::Format>>,
        Vec<hal::PresentMode>,
    ) {
        unimplemented!()
    }

    fn supports_queue_family(&self, _: &QueueFamily) -> bool {
        unimplemented!()
    }
}

pub type SurfaceImage = native::ImageView;

impl hal::PresentationSurface<Backend> for Surface {
    type SwapchainImage = native::ImageView;

    unsafe fn configure_swapchain(
        &mut self, _: &Device, _: hal::SwapchainConfig
    ) -> Result<(), hal::window::CreationError> {
        unimplemented!()
    }

    unsafe fn acquire_image(
        &mut self,
        _: u64,
    ) -> Result<(Self::SwapchainImage, Option<hal::window::Suboptimal>), hal::AcquireError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Swapchain {
    pub(crate) extent: hal::window::Extent2D,
    pub(crate) fbos: ArrayVec<[native::RawFrameBuffer; 0]>,
}

impl hal::Swapchain<Backend> for Swapchain {
    unsafe fn acquire_image(
        &mut self,
        _: u64,
        _: Option<&native::Semaphore>,
        _: Option<&native::Fence>,
    ) -> Result<(hal::SwapImageIndex, Option<hal::window::Suboptimal>), hal::AcquireError> {
        unimplemented!()
    }
}
