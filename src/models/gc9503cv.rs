use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_hal::delay::DelayNs;

use crate::{
    dcs::{
        BitsPerPixel, ExitSleepMode, InterfaceExt, PixelFormat, SetAddressMode, SetDisplayOn,
        SetInvertMode, SetPixelFormat,
    },
    interface::{Interface, InterfaceKind},
    models::{Model, ModelInitError},
    options::ModelOptions,
    ConfigurationError,
};

/// GC9503CV display in Rgb565 color mode.
pub struct GC9503CV;

impl Model for GC9503CV {
    type ColorFormat = Rgb565;
    const FRAMEBUFFER_SIZE: (u16, u16) = (480, 480);

    fn init<DELAY, DI>(
        &mut self,
        di: &mut DI,
        delay: &mut DELAY,
        options: &ModelOptions,
    ) -> Result<SetAddressMode, ModelInitError<DI::Error>>
    where
        DELAY: DelayNs,
        DI: Interface,
    {
        if !matches!(
            DI::KIND,
            InterfaceKind::Serial4Line | InterfaceKind::Parallel8Bit
        ) {
            return Err(ModelInitError::InvalidConfiguration(
                ConfigurationError::UnsupportedInterface,
            ));
        }

        // Initial delay
        delay.delay_ms(120);

        // Software reset
        di.write_raw(0x01, &[])?;
        delay.delay_ms(120);

        // Sleep out
        di.write_command(ExitSleepMode)?;
        delay.delay_ms(120);

        // Set frame rate
        di.write_raw(0xB1, &[0x02, 0x35, 0x36])?;
        
        // Set panel driving mode
        di.write_raw(0xB4, &[0x00])?;
        
        // Set display inversion
        di.write_raw(0xB7, &[0x02])?;
        
        // Set power control
        di.write_raw(0xC0, &[0x18, 0x18])?;
        di.write_raw(0xC1, &[0x41])?;
        di.write_raw(0xC2, &[0x22])?;
        
        // Vcom voltage
        di.write_raw(0xC5, &[0x30])?;
        
        // Memory access control (MADCTL)
        let madctl = SetAddressMode::from(options);
        di.write_command(madctl)?;
        
        // Pixel format
        let pf = PixelFormat::with_all(BitsPerPixel::from_rgb_color::<Self::ColorFormat>());
        di.write_command(SetPixelFormat::new(pf))?;
        
        // Gamma correction
        di.write_raw(0xE0, &[0x1F, 0x25, 0x22, 0x0B, 0x06, 0x0A, 0x4E, 0xC6, 0x39, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?;
        di.write_raw(0xE1, &[0x1F, 0x3F, 0x3F, 0x0F, 0x1F, 0x0F, 0x46, 0x49, 0x3B, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x00])?;
        
        // Set inversion mode
        di.write_command(SetInvertMode::new(options.invert_colors))?;
        
        // Turn on display
        di.write_command(SetDisplayOn)?;
        delay.delay_ms(20);

        Ok(madctl)
    }
}
