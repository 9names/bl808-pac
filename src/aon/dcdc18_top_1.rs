#[doc = "Register `dcdc18_top_1` reader"]
pub struct R(crate::R<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC18_TOP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC18_TOP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC18_TOP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc18_top_1` writer"]
pub struct W(crate::W<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC18_TOP_1_SPEC>;
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
impl From<crate::W<DCDC18_TOP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC18_TOP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc18_nonoverlap_td_aon` reader - "]
pub type DCDC18_NONOVERLAP_TD_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_nonoverlap_td_aon` writer - "]
pub type DCDC18_NONOVERLAP_TD_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `dcdc18_ocp_out_aon` reader - "]
pub type DCDC18_OCP_OUT_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_ocp_rst_aon` reader - "]
pub type DCDC18_OCP_RST_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_ocp_rst_aon` writer - "]
pub type DCDC18_OCP_RST_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_ocp_vth_aon` reader - "]
pub type DCDC18_OCP_VTH_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_ocp_vth_aon` writer - "]
pub type DCDC18_OCP_VTH_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `dcdc18_osc_2m_mode_aon` reader - "]
pub type DCDC18_OSC_2M_MODE_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_osc_2m_mode_aon` writer - "]
pub type DCDC18_OSC_2M_MODE_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
#[doc = "Field `dcdc18_osc_freq_trim_aon` reader - "]
pub type DCDC18_OSC_FREQ_TRIM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_osc_freq_trim_aon` writer - "]
pub type DCDC18_OSC_FREQ_TRIM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_pulldown_aon` reader - "]
pub type DCDC18_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_pulldown_aon` writer - "]
pub type DCDC18_PULLDOWN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC18_TOP_1_SPEC, bool, O>;
#[doc = "Field `reserved_17_19` reader - "]
pub type RESERVED_17_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_rc_sel_aon` reader - "]
pub type DCDC18_RC_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_rc_sel_aon` writer - "]
pub type DCDC18_RC_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc18_rdy_aon` reader - "]
pub type DCDC18_RDY_AON_R = crate::BitReader<bool>;
#[doc = "Field `reserved_25` reader - "]
pub type RESERVED_25_R = crate::BitReader<bool>;
#[doc = "Field `dcdc18_slope_curr_sel_aon` reader - "]
pub type DCDC18_SLOPE_CURR_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc18_slope_curr_sel_aon` writer - "]
pub type DCDC18_SLOPE_CURR_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC18_TOP_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> DCDC18_NONOVERLAP_TD_AON_R {
        DCDC18_NONOVERLAP_TD_AON_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dcdc18_ocp_out_aon(&self) -> DCDC18_OCP_OUT_AON_R {
        DCDC18_OCP_OUT_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dcdc18_ocp_rst_aon(&self) -> DCDC18_OCP_RST_AON_R {
        DCDC18_OCP_RST_AON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dcdc18_ocp_vth_aon(&self) -> DCDC18_OCP_VTH_AON_R {
        DCDC18_OCP_VTH_AON_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&self) -> DCDC18_OSC_2M_MODE_AON_R {
        DCDC18_OSC_2M_MODE_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&self) -> DCDC18_OSC_FREQ_TRIM_AON_R {
        DCDC18_OSC_FREQ_TRIM_AON_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> DCDC18_PULLDOWN_AON_R {
        DCDC18_PULLDOWN_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn reserved_17_19(&self) -> RESERVED_17_19_R {
        RESERVED_17_19_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> DCDC18_RC_SEL_AON_R {
        DCDC18_RC_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&self) -> DCDC18_RDY_AON_R {
        DCDC18_RDY_AON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reserved_25(&self) -> RESERVED_25_R {
        RESERVED_25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&self) -> DCDC18_SLOPE_CURR_SEL_AON_R {
        DCDC18_SLOPE_CURR_SEL_AON_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> DCDC18_NONOVERLAP_TD_AON_W<0> {
        DCDC18_NONOVERLAP_TD_AON_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_ocp_rst_aon(&mut self) -> DCDC18_OCP_RST_AON_W<6> {
        DCDC18_OCP_RST_AON_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_ocp_vth_aon(&mut self) -> DCDC18_OCP_VTH_AON_W<8> {
        DCDC18_OCP_VTH_AON_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_osc_2m_mode_aon(&mut self) -> DCDC18_OSC_2M_MODE_AON_W<11> {
        DCDC18_OSC_2M_MODE_AON_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_osc_freq_trim_aon(&mut self) -> DCDC18_OSC_FREQ_TRIM_AON_W<12> {
        DCDC18_OSC_FREQ_TRIM_AON_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_pulldown_aon(&mut self) -> DCDC18_PULLDOWN_AON_W<16> {
        DCDC18_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_rc_sel_aon(&mut self) -> DCDC18_RC_SEL_AON_W<20> {
        DCDC18_RC_SEL_AON_W::new(self)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc18_slope_curr_sel_aon(&mut self) -> DCDC18_SLOPE_CURR_SEL_AON_W<26> {
        DCDC18_SLOPE_CURR_SEL_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dcdc18_top_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_1](index.html) module"]
pub struct DCDC18_TOP_1_SPEC;
impl crate::RegisterSpec for DCDC18_TOP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc18_top_1::R](R) reader structure"]
impl crate::Readable for DCDC18_TOP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_1::W](W) writer structure"]
impl crate::Writable for DCDC18_TOP_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc18_top_1 to value 0x2880_8400"]
impl crate::Resettable for DCDC18_TOP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2880_8400;
}
