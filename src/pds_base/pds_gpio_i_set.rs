#[doc = "Register `pds_gpio_i_set` reader"]
pub struct R(crate::R<PDS_GPIO_I_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_I_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_I_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_I_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_i_set` writer"]
pub struct W(crate::W<PDS_GPIO_I_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_I_SET_SPEC>;
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
impl From<crate::W<PDS_GPIO_I_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_I_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_np_wfi_mask` reader - "]
pub type CR_NP_WFI_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_np_wfi_mask` writer - "]
pub type CR_NP_WFI_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_I_SET_SPEC, bool, O>;
#[doc = "Field `reserved_1` reader - "]
pub type RESERVED_1_R = crate::BitReader<bool>;
#[doc = "Field `cr_mm_wfi_mask` reader - "]
pub type CR_MM_WFI_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_mm_wfi_mask` writer - "]
pub type CR_MM_WFI_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_I_SET_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `cr_pico_wfi_mask` reader - "]
pub type CR_PICO_WFI_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pico_wfi_mask` writer - "]
pub type CR_PICO_WFI_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_I_SET_SPEC, bool, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ctrl_usb33` reader - "]
pub type CR_PDS_CTRL_USB33_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ctrl_usb33` writer - "]
pub type CR_PDS_CTRL_USB33_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_I_SET_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_ldo18io` reader - "]
pub type CR_PDS_PD_LDO18IO_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_ldo18io` writer - "]
pub type CR_PDS_PD_LDO18IO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_I_SET_SPEC, bool, O>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_gpio_keep_en` reader - "]
pub type CR_PDS_GPIO_KEEP_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_gpio_keep_en` writer - "]
pub type CR_PDS_GPIO_KEEP_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_I_SET_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_31` reader - "]
pub type RESERVED_19_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_np_wfi_mask(&self) -> CR_NP_WFI_MASK_R {
        CR_NP_WFI_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reserved_1(&self) -> RESERVED_1_R {
        RESERVED_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mm_wfi_mask(&self) -> CR_MM_WFI_MASK_R {
        CR_MM_WFI_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pico_wfi_mask(&self) -> CR_PICO_WFI_MASK_R {
        CR_PICO_WFI_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_ctrl_usb33(&self) -> CR_PDS_CTRL_USB33_R {
        CR_PDS_CTRL_USB33_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pd_ldo18io(&self) -> CR_PDS_PD_LDO18IO_R {
        CR_PDS_PD_LDO18IO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cr_pds_gpio_keep_en(&self) -> CR_PDS_GPIO_KEEP_EN_R {
        CR_PDS_GPIO_KEEP_EN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn reserved_19_31(&self) -> RESERVED_19_31_R {
        RESERVED_19_31_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_np_wfi_mask(&mut self) -> CR_NP_WFI_MASK_W<0> {
        CR_NP_WFI_MASK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mm_wfi_mask(&mut self) -> CR_MM_WFI_MASK_W<2> {
        CR_MM_WFI_MASK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pico_wfi_mask(&mut self) -> CR_PICO_WFI_MASK_W<4> {
        CR_PICO_WFI_MASK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_usb33(&mut self) -> CR_PDS_CTRL_USB33_W<8> {
        CR_PDS_CTRL_USB33_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_ldo18io(&mut self) -> CR_PDS_PD_LDO18IO_W<9> {
        CR_PDS_PD_LDO18IO_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_gpio_keep_en(&mut self) -> CR_PDS_GPIO_KEEP_EN_W<16> {
        CR_PDS_GPIO_KEEP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_i_set](index.html) module"]
pub struct PDS_GPIO_I_SET_SPEC;
impl crate::RegisterSpec for PDS_GPIO_I_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_i_set::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_I_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_i_set::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_I_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_i_set to value 0x0007_0000"]
impl crate::Resettable for PDS_GPIO_I_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0000;
}
