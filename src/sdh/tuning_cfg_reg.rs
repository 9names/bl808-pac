#[doc = "Register `tuning_cfg_reg` reader"]
pub struct R(crate::R<TUNING_CFG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUNING_CFG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUNING_CFG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUNING_CFG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tuning_cfg_reg` writer"]
pub struct W(crate::W<TUNING_CFG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUNING_CFG_REG_SPEC>;
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
impl From<crate::W<TUNING_CFG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUNING_CFG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tuning_tt_cnt` reader - "]
pub type TUNING_TT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tuning_tt_cnt` writer - "]
pub type TUNING_TT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CFG_REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `tuning_wd_cnt` reader - "]
pub type TUNING_WD_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tuning_wd_cnt` writer - "]
pub type TUNING_WD_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TUNING_CFG_REG_SPEC, u8, u8, 6, O>;
#[doc = "Field `tuning_clk_dly` reader - "]
pub type TUNING_CLK_DLY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tuning_tt_cnt(&self) -> TUNING_TT_CNT_R {
        TUNING_TT_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tuning_wd_cnt(&self) -> TUNING_WD_CNT_R {
        TUNING_WD_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:23"]
    #[inline(always)]
    pub fn tuning_clk_dly(&self) -> TUNING_CLK_DLY_R {
        TUNING_CLK_DLY_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_tt_cnt(&mut self) -> TUNING_TT_CNT_W<0> {
        TUNING_TT_CNT_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_wd_cnt(&mut self) -> TUNING_WD_CNT_W<8> {
        TUNING_WD_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TUNING CONFIG Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_cfg_reg](index.html) module"]
pub struct TUNING_CFG_REG_SPEC;
impl crate::RegisterSpec for TUNING_CFG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tuning_cfg_reg::R](R) reader structure"]
impl crate::Readable for TUNING_CFG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tuning_cfg_reg::W](W) writer structure"]
impl crate::Writable for TUNING_CFG_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tuning_cfg_reg to value 0x0a27"]
impl crate::Resettable for TUNING_CFG_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a27;
}
