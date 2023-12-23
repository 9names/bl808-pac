#[doc = "Register `preset_value_for_sdr104` reader"]
pub struct R(crate::R<PRESET_VALUE_FOR_SDR104_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESET_VALUE_FOR_SDR104_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESET_VALUE_FOR_SDR104_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESET_VALUE_FOR_SDR104_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `preset_value_for_sdr104` writer"]
pub struct W(crate::W<PRESET_VALUE_FOR_SDR104_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESET_VALUE_FOR_SDR104_SPEC>;
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
impl From<crate::W<PRESET_VALUE_FOR_SDR104_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESET_VALUE_FOR_SDR104_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdclk_freq_sel_val` reader - "]
pub type SDCLK_FREQ_SEL_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `clkgen_sel_val` reader - "]
pub type CLKGEN_SEL_VAL_R = crate::BitReader<bool>;
#[doc = "Field `reserved_13_11` reader - "]
pub type RESERVED_13_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drv_strength_val` reader - "]
pub type DRV_STRENGTH_VAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sdclk_freq_sel_val(&self) -> SDCLK_FREQ_SEL_VAL_R {
        SDCLK_FREQ_SEL_VAL_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkgen_sel_val(&self) -> CLKGEN_SEL_VAL_R {
        CLKGEN_SEL_VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn reserved_13_11(&self) -> RESERVED_13_11_R {
        RESERVED_13_11_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn drv_strength_val(&self) -> DRV_STRENGTH_VAL_R {
        DRV_STRENGTH_VAL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Preset Value Register for SDR104\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_value_for_sdr104](index.html) module"]
pub struct PRESET_VALUE_FOR_SDR104_SPEC;
impl crate::RegisterSpec for PRESET_VALUE_FOR_SDR104_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [preset_value_for_sdr104::R](R) reader structure"]
impl crate::Readable for PRESET_VALUE_FOR_SDR104_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preset_value_for_sdr104::W](W) writer structure"]
impl crate::Writable for PRESET_VALUE_FOR_SDR104_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets preset_value_for_sdr104 to value 0"]
impl crate::Resettable for PRESET_VALUE_FOR_SDR104_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
