#[doc = "Register `dcdc18_top_0` reader"]
pub struct R(crate::R<DCDC18_TOP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC18_TOP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC18_TOP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC18_TOP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc18_top_0` writer"]
pub struct W(crate::W<DCDC18_TOP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC18_TOP_0_SPEC>;
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
impl From<crate::W<DCDC18_TOP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC18_TOP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc18_sstart_time_aon` reader - "]
pub type DCDC18_SSTART_TIME_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_sstart_time_aon` writer - "]
pub type DCDC18_SSTART_TIME_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_3` reader - "]
pub type RESERVED_2_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_stby_lp_cur_aon` reader - "]
pub type DCDC18_STBY_LP_CUR_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_stby_lp_cur_aon` writer - "]
pub type DCDC18_STBY_LP_CUR_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_vc_clamp_vth_aon` reader - "]
pub type DCDC18_VC_CLAMP_VTH_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_vc_clamp_vth_aon` writer - "]
pub type DCDC18_VC_CLAMP_VTH_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `dcdc18_vout_sel_aon` reader - "]
pub type DCDC18_VOUT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_vout_sel_aon` writer - "]
pub type DCDC18_VOUT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `dcdc18_vout_trim_aon` reader - "]
pub type DCDC18_VOUT_TRIM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_vout_trim_aon` writer - "]
pub type DCDC18_VOUT_TRIM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_vpfm_aon` reader - "]
pub type DCDC18_VPFM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_vpfm_aon` writer - "]
pub type DCDC18_VPFM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` reader - "]
pub type DCDC18_ZVS_TD_OPT_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_zvs_td_opt_aon` writer - "]
pub type DCDC18_ZVS_TD_OPT_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_27` reader - "]
pub type RESERVED_27_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_vstby_aon` reader - "]
pub type DCDC18_VSTBY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_vstby_aon` writer - "]
pub type DCDC18_VSTBY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&self) -> DCDC18_SSTART_TIME_AON_R {
        DCDC18_SSTART_TIME_AON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reserved_2_3(&self) -> RESERVED_2_3_R {
        RESERVED_2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_stby_lp_cur_aon(&self) -> DCDC18_STBY_LP_CUR_AON_R {
        DCDC18_STBY_LP_CUR_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_vc_clamp_vth_aon(&self) -> DCDC18_VC_CLAMP_VTH_AON_R {
        DCDC18_VC_CLAMP_VTH_AON_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&self) -> DCDC18_VOUT_SEL_AON_R {
        DCDC18_VOUT_SEL_AON_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_vout_trim_aon(&self) -> DCDC18_VOUT_TRIM_AON_R {
        DCDC18_VOUT_TRIM_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&self) -> DCDC18_VPFM_AON_R {
        DCDC18_VPFM_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> DCDC18_ZVS_TD_OPT_AON_R {
        DCDC18_ZVS_TD_OPT_AON_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reserved_27(&self) -> RESERVED_27_R {
        RESERVED_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_vstby_aon(&self) -> DCDC18_VSTBY_AON_R {
        DCDC18_VSTBY_AON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_sstart_time_aon(&mut self) -> DCDC18_SSTART_TIME_AON_W<0> {
        DCDC18_SSTART_TIME_AON_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_stby_lp_cur_aon(&mut self) -> DCDC18_STBY_LP_CUR_AON_W<4> {
        DCDC18_STBY_LP_CUR_AON_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_vc_clamp_vth_aon(&mut self) -> DCDC18_VC_CLAMP_VTH_AON_W<8> {
        DCDC18_VC_CLAMP_VTH_AON_W::new(self)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_vout_sel_aon(&mut self) -> DCDC18_VOUT_SEL_AON_W<11> {
        DCDC18_VOUT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_vout_trim_aon(&mut self) -> DCDC18_VOUT_TRIM_AON_W<16> {
        DCDC18_VOUT_TRIM_AON_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_vpfm_aon(&mut self) -> DCDC18_VPFM_AON_W<20> {
        DCDC18_VPFM_AON_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> DCDC18_ZVS_TD_OPT_AON_W<24> {
        DCDC18_ZVS_TD_OPT_AON_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_vstby_aon(&mut self) -> DCDC18_VSTBY_AON_W<28> {
        DCDC18_VSTBY_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "move to 0x2000F000\\[23\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_0](index.html) module"]
pub struct DCDC18_TOP_0_SPEC;
impl crate::RegisterSpec for DCDC18_TOP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc18_top_0::R](R) reader structure"]
impl crate::Readable for DCDC18_TOP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_0::W](W) writer structure"]
impl crate::Writable for DCDC18_TOP_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc18_top_0 to value 0x1447_dc20"]
impl crate::Resettable for DCDC18_TOP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1447_dc20;
}
