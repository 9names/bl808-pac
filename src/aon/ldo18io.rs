#[doc = "Register `ldo18io` reader"]
pub struct R(crate::R<LDO18IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO18IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO18IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO18IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo18io` writer"]
pub struct W(crate::W<LDO18IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO18IO_SPEC>;
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
impl From<crate::W<LDO18IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO18IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0` reader - "]
pub type RESERVED_0_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_bypass_iso_aon` reader - "]
pub type LDO18IO_BYPASS_ISO_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_bypass_iso_aon` writer - "]
pub type LDO18IO_BYPASS_ISO_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_pulldown_aon` reader - "]
pub type LDO18IO_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_pulldown_aon` writer - "]
pub type LDO18IO_PULLDOWN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_pulldown_sel_aon` reader - "]
pub type LDO18IO_PULLDOWN_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_pulldown_sel_aon` writer - "]
pub type LDO18IO_PULLDOWN_SEL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_bm_aon` reader - "]
pub type LDO18IO_BM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_bm_aon` writer - "]
pub type LDO18IO_BM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_7` reader - "]
pub type RESERVED_7_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_cc_aon` reader - "]
pub type LDO18IO_CC_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_cc_aon` writer - "]
pub type LDO18IO_CC_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18io_ocp_out_aon` reader - "]
pub type LDO18IO_OCP_OUT_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_ocp_th_aon` reader - "]
pub type LDO18IO_OCP_TH_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_ocp_th_aon` writer - "]
pub type LDO18IO_OCP_TH_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18io_ocp_en_aon` reader - "]
pub type LDO18IO_OCP_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_ocp_en_aon` writer - "]
pub type LDO18IO_OCP_EN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_sstart_delay_aon` reader - "]
pub type LDO18IO_SSTART_DELAY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_sstart_delay_aon` writer - "]
pub type LDO18IO_SSTART_DELAY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 3, O>;
#[doc = "Field `ldo18io_sstart_en_aon` reader - "]
pub type LDO18IO_SSTART_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo18io_sstart_en_aon` writer - "]
pub type LDO18IO_SSTART_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO18IO_SPEC, bool, O>;
#[doc = "Field `ldo18io_vout_sel_aon` reader - "]
pub type LDO18IO_VOUT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_vout_sel_aon` writer - "]
pub type LDO18IO_VOUT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 4, O>;
#[doc = "Field `ldo18io_vout_trim_aon` reader - "]
pub type LDO18IO_VOUT_TRIM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo18io_vout_trim_aon` writer - "]
pub type LDO18IO_VOUT_TRIM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO18IO_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> RESERVED_0_R {
        RESERVED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18io_bypass_iso_aon(&self) -> LDO18IO_BYPASS_ISO_AON_R {
        LDO18IO_BYPASS_ISO_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18io_pulldown_aon(&self) -> LDO18IO_PULLDOWN_AON_R {
        LDO18IO_PULLDOWN_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18io_pulldown_sel_aon(&self) -> LDO18IO_PULLDOWN_SEL_AON_R {
        LDO18IO_PULLDOWN_SEL_AON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18io_bm_aon(&self) -> LDO18IO_BM_AON_R {
        LDO18IO_BM_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18io_cc_aon(&self) -> LDO18IO_CC_AON_R {
        LDO18IO_CC_AON_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo18io_ocp_out_aon(&self) -> LDO18IO_OCP_OUT_AON_R {
        LDO18IO_OCP_OUT_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18io_ocp_th_aon(&self) -> LDO18IO_OCP_TH_AON_R {
        LDO18IO_OCP_TH_AON_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18io_ocp_en_aon(&self) -> LDO18IO_OCP_EN_AON_R {
        LDO18IO_OCP_EN_AON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18io_sstart_delay_aon(&self) -> LDO18IO_SSTART_DELAY_AON_R {
        LDO18IO_SSTART_DELAY_AON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18io_sstart_en_aon(&self) -> LDO18IO_SSTART_EN_AON_R {
        LDO18IO_SSTART_EN_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18io_vout_sel_aon(&self) -> LDO18IO_VOUT_SEL_AON_R {
        LDO18IO_VOUT_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18io_vout_trim_aon(&self) -> LDO18IO_VOUT_TRIM_AON_R {
        LDO18IO_VOUT_TRIM_AON_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_bypass_iso_aon(&mut self) -> LDO18IO_BYPASS_ISO_AON_W<1> {
        LDO18IO_BYPASS_ISO_AON_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_pulldown_aon(&mut self) -> LDO18IO_PULLDOWN_AON_W<2> {
        LDO18IO_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_pulldown_sel_aon(&mut self) -> LDO18IO_PULLDOWN_SEL_AON_W<3> {
        LDO18IO_PULLDOWN_SEL_AON_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_bm_aon(&mut self) -> LDO18IO_BM_AON_W<4> {
        LDO18IO_BM_AON_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_cc_aon(&mut self) -> LDO18IO_CC_AON_W<8> {
        LDO18IO_CC_AON_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_ocp_th_aon(&mut self) -> LDO18IO_OCP_TH_AON_W<12> {
        LDO18IO_OCP_TH_AON_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_ocp_en_aon(&mut self) -> LDO18IO_OCP_EN_AON_W<15> {
        LDO18IO_OCP_EN_AON_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_sstart_delay_aon(&mut self) -> LDO18IO_SSTART_DELAY_AON_W<16> {
        LDO18IO_SSTART_DELAY_AON_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_sstart_en_aon(&mut self) -> LDO18IO_SSTART_EN_AON_W<19> {
        LDO18IO_SSTART_EN_AON_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_vout_sel_aon(&mut self) -> LDO18IO_VOUT_SEL_AON_W<20> {
        LDO18IO_VOUT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_vout_trim_aon(&mut self) -> LDO18IO_VOUT_TRIM_AON_W<24> {
        LDO18IO_VOUT_TRIM_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo18io\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo18io](index.html) module"]
pub struct LDO18IO_SPEC;
impl crate::RegisterSpec for LDO18IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo18io::R](R) reader structure"]
impl crate::Readable for LDO18IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo18io::W](W) writer structure"]
impl crate::Writable for LDO18IO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18io to value 0x075b_b338"]
impl crate::Resettable for LDO18IO_SPEC {
    const RESET_VALUE: Self::Ux = 0x075b_b338;
}
