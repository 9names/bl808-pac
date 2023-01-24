#[doc = "Register `dcdc_top_0` reader"]
pub struct R(crate::R<DCDC_TOP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_TOP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_TOP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_TOP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc_top_0` writer"]
pub struct W(crate::W<DCDC_TOP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_TOP_0_SPEC>;
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
impl From<crate::W<DCDC_TOP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_TOP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc11_sstart_time_aon` reader - "]
pub type DCDC11_SSTART_TIME_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_sstart_time_aon` writer - "]
pub type DCDC11_SSTART_TIME_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_TOP_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_stby_lp_cur_aon` reader - "]
pub type DCDC11_STBY_LP_CUR_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_stby_lp_cur_aon` writer - "]
pub type DCDC11_STBY_LP_CUR_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_TOP_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc11_sstart_time_aon(&self) -> DCDC11_SSTART_TIME_AON_R {
        DCDC11_SSTART_TIME_AON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc11_stby_lp_cur_aon(&self) -> DCDC11_STBY_LP_CUR_AON_R {
        DCDC11_STBY_LP_CUR_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_sstart_time_aon(&mut self) -> DCDC11_SSTART_TIME_AON_W<0> {
        DCDC11_SSTART_TIME_AON_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_stby_lp_cur_aon(&mut self) -> DCDC11_STBY_LP_CUR_AON_W<4> {
        DCDC11_STBY_LP_CUR_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dcdc_top_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_top_0](index.html) module"]
pub struct DCDC_TOP_0_SPEC;
impl crate::RegisterSpec for DCDC_TOP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_top_0::R](R) reader structure"]
impl crate::Readable for DCDC_TOP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_top_0::W](W) writer structure"]
impl crate::Writable for DCDC_TOP_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc_top_0 to value 0x20"]
impl crate::Resettable for DCDC_TOP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
