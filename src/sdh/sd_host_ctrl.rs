#[doc = "Register `sd_host_ctrl` reader"]
pub struct R(crate::R<SD_HOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_HOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_HOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_HOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_host_ctrl` writer"]
pub struct W(crate::W<SD_HOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_HOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_HOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_HOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `led_ctrl` reader - "]
pub type LED_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `led_ctrl` writer - "]
pub type LED_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `data_width` reader - "]
pub type DATA_WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `data_width` writer - "]
pub type DATA_WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `hi_speed_en` reader - "]
pub type HI_SPEED_EN_R = crate::BitReader<bool>;
#[doc = "Field `hi_speed_en` writer - "]
pub type HI_SPEED_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `dma_sel` reader - "]
pub type DMA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dma_sel` writer - "]
pub type DMA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SD_HOST_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ex_data_width` reader - "]
pub type EX_DATA_WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `ex_data_width` writer - "]
pub type EX_DATA_WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `card_det_l` reader - "]
pub type CARD_DET_L_R = crate::BitReader<bool>;
#[doc = "Field `card_det_l` writer - "]
pub type CARD_DET_L_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `card_det_s` reader - "]
pub type CARD_DET_S_R = crate::BitReader<bool>;
#[doc = "Field `card_det_s` writer - "]
pub type CARD_DET_S_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `sd_bus_power` reader - "]
pub type SD_BUS_POWER_R = crate::BitReader<bool>;
#[doc = "Field `sd_bus_power` writer - "]
pub type SD_BUS_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `sd_bus_vlt` reader - "]
pub type SD_BUS_VLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sd_bus_vlt` writer - "]
pub type SD_BUS_VLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_HOST_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15_12` reader - "]
pub type RESERVED_15_12_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LED_CTRL_R {
        LED_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hi_speed_en(&self) -> HI_SPEED_EN_R {
        HI_SPEED_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DMA_SEL_R {
        DMA_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ex_data_width(&self) -> EX_DATA_WIDTH_R {
        EX_DATA_WIDTH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_det_l(&self) -> CARD_DET_L_R {
        CARD_DET_L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_det_s(&self) -> CARD_DET_S_R {
        CARD_DET_S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SD_BUS_POWER_R {
        SD_BUS_POWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn sd_bus_vlt(&self) -> SD_BUS_VLT_R {
        SD_BUS_VLT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_15_12(&self) -> RESERVED_15_12_R {
        RESERVED_15_12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl(&mut self) -> LED_CTRL_W<0> {
        LED_CTRL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<1> {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hi_speed_en(&mut self) -> HI_SPEED_EN_W<2> {
        HI_SPEED_EN_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel(&mut self) -> DMA_SEL_W<3> {
        DMA_SEL_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ex_data_width(&mut self) -> EX_DATA_WIDTH_W<5> {
        EX_DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn card_det_l(&mut self) -> CARD_DET_L_W<6> {
        CARD_DET_L_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn card_det_s(&mut self) -> CARD_DET_S_W<7> {
        CARD_DET_S_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_power(&mut self) -> SD_BUS_POWER_W<8> {
        SD_BUS_POWER_W::new(self)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_vlt(&mut self) -> SD_BUS_VLT_W<9> {
        SD_BUS_VLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_host_ctrl](index.html) module"]
pub struct SD_HOST_CTRL_SPEC;
impl crate::RegisterSpec for SD_HOST_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_host_ctrl::R](R) reader structure"]
impl crate::Readable for SD_HOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_host_ctrl::W](W) writer structure"]
impl crate::Writable for SD_HOST_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_host_ctrl to value 0"]
impl crate::Resettable for SD_HOST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
