#[doc = "Register `host_ctrl_2` reader"]
pub struct R(crate::R<HOST_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `host_ctrl_2` writer"]
pub struct W(crate::W<HOST_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTRL_2_SPEC>;
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
impl From<crate::W<HOST_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uhs_mode_sel` reader - "]
pub type UHS_MODE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uhs_mode_sel` writer - "]
pub type UHS_MODE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, HOST_CTRL_2_SPEC, u8, u8, 3, O>;
#[doc = "Field `sdh_v18_en` reader - "]
pub type SDH_V18_EN_R = crate::BitReader<bool>;
#[doc = "Field `sdh_v18_en` writer - "]
pub type SDH_V18_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, HOST_CTRL_2_SPEC, bool, O>;
#[doc = "Field `drv_strength_sel` reader - "]
pub type DRV_STRENGTH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drv_strength_sel` writer - "]
pub type DRV_STRENGTH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, HOST_CTRL_2_SPEC, u8, u8, 2, O>;
#[doc = "Field `exe_tuning` reader - "]
pub type EXE_TUNING_R = crate::BitReader<bool>;
#[doc = "Field `exe_tuning` writer - "]
pub type EXE_TUNING_W<'a, const O: u8> = crate::BitWriter<'a, u16, HOST_CTRL_2_SPEC, bool, O>;
#[doc = "Field `sampling_clk_sel` reader - "]
pub type SAMPLING_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `sampling_clk_sel` writer - "]
pub type SAMPLING_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, HOST_CTRL_2_SPEC, bool, O>;
#[doc = "Field `reserved_13_8` reader - "]
pub type RESERVED_13_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `async_int_en` reader - "]
pub type ASYNC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `async_int_en` writer - "]
pub type ASYNC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, HOST_CTRL_2_SPEC, bool, O>;
#[doc = "Field `pre_val_en` reader - "]
pub type PRE_VAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `pre_val_en` writer - "]
pub type PRE_VAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, HOST_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhs_mode_sel(&self) -> UHS_MODE_SEL_R {
        UHS_MODE_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sdh_v18_en(&self) -> SDH_V18_EN_R {
        SDH_V18_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn drv_strength_sel(&self) -> DRV_STRENGTH_SEL_R {
        DRV_STRENGTH_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn exe_tuning(&self) -> EXE_TUNING_R {
        EXE_TUNING_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sampling_clk_sel(&self) -> SAMPLING_CLK_SEL_R {
        SAMPLING_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reserved_13_8(&self) -> RESERVED_13_8_R {
        RESERVED_13_8_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn async_int_en(&self) -> ASYNC_INT_EN_R {
        ASYNC_INT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pre_val_en(&self) -> PRE_VAL_EN_R {
        PRE_VAL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn uhs_mode_sel(&mut self) -> UHS_MODE_SEL_W<0> {
        UHS_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sdh_v18_en(&mut self) -> SDH_V18_EN_W<3> {
        SDH_V18_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn drv_strength_sel(&mut self) -> DRV_STRENGTH_SEL_W<4> {
        DRV_STRENGTH_SEL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn exe_tuning(&mut self) -> EXE_TUNING_W<6> {
        EXE_TUNING_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sampling_clk_sel(&mut self) -> SAMPLING_CLK_SEL_W<7> {
        SAMPLING_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn async_int_en(&mut self) -> ASYNC_INT_EN_W<14> {
        ASYNC_INT_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pre_val_en(&mut self) -> PRE_VAL_EN_W<15> {
        PRE_VAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl_2](index.html) module"]
pub struct HOST_CTRL_2_SPEC;
impl crate::RegisterSpec for HOST_CTRL_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [host_ctrl_2::R](R) reader structure"]
impl crate::Readable for HOST_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctrl_2::W](W) writer structure"]
impl crate::Writable for HOST_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets host_ctrl_2 to value 0x4000"]
impl crate::Resettable for HOST_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
