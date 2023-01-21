#[doc = "Register `rf_top_aon` reader"]
pub struct R(crate::R<RF_TOP_AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TOP_AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_TOP_AON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_TOP_AON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_top_aon` writer"]
pub struct W(crate::W<RF_TOP_AON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TOP_AON_SPEC>;
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
impl From<crate::W<RF_TOP_AON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_TOP_AON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_mbg_aon` reader - "]
pub type PU_MBG_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_mbg_aon` writer - "]
pub type PU_MBG_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `pu_ldo15rf_aon` reader - "]
pub type PU_LDO15RF_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo15rf_aon` writer - "]
pub type PU_LDO15RF_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `pu_sfreg_aon` reader - "]
pub type PU_SFREG_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_sfreg_aon` writer - "]
pub type PU_SFREG_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `pu_xtal_buf_aon` reader - "]
pub type PU_XTAL_BUF_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_xtal_buf_aon` writer - "]
pub type PU_XTAL_BUF_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `pu_xtal_aon` reader - "]
pub type PU_XTAL_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_xtal_aon` writer - "]
pub type PU_XTAL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_sstart_sel_aon` reader - "]
pub type LDO15RF_SSTART_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo15rf_sstart_sel_aon` writer - "]
pub type LDO15RF_SSTART_SEL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `ldo15rf_sstart_delay_aon` reader - "]
pub type LDO15RF_SSTART_DELAY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_sstart_delay_aon` writer - "]
pub type LDO15RF_SSTART_DELAY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TOP_AON_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `ldo15rf_pulldown_aon` reader - "]
pub type LDO15RF_PULLDOWN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo15rf_pulldown_aon` writer - "]
pub type LDO15RF_PULLDOWN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `ldo15rf_pulldown_sel_aon` reader - "]
pub type LDO15RF_PULLDOWN_SEL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo15rf_pulldown_sel_aon` writer - "]
pub type LDO15RF_PULLDOWN_SEL_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_vout_sel_aon` reader - "]
pub type LDO15RF_VOUT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_vout_sel_aon` writer - "]
pub type LDO15RF_VOUT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TOP_AON_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_cc_aon` reader - "]
pub type LDO15RF_CC_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_cc_aon` writer - "]
pub type LDO15RF_CC_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TOP_AON_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo15rf_bypass_aon` reader - "]
pub type LDO15RF_BYPASS_AON_R = crate::BitReader<bool>;
#[doc = "Field `ldo15rf_bypass_aon` writer - "]
pub type LDO15RF_BYPASS_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_TOP_AON_SPEC, bool, O>;
#[doc = "Field `reserved_29_31` reader - "]
pub type RESERVED_29_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&self) -> PU_MBG_AON_R {
        PU_MBG_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&self) -> PU_LDO15RF_AON_R {
        PU_LDO15RF_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&self) -> PU_SFREG_AON_R {
        PU_SFREG_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&self) -> PU_XTAL_BUF_AON_R {
        PU_XTAL_BUF_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&self) -> PU_XTAL_AON_R {
        PU_XTAL_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&self) -> LDO15RF_SSTART_SEL_AON_R {
        LDO15RF_SSTART_SEL_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&self) -> LDO15RF_SSTART_DELAY_AON_R {
        LDO15RF_SSTART_DELAY_AON_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&self) -> LDO15RF_PULLDOWN_AON_R {
        LDO15RF_PULLDOWN_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&self) -> LDO15RF_PULLDOWN_SEL_AON_R {
        LDO15RF_PULLDOWN_SEL_AON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&self) -> LDO15RF_VOUT_SEL_AON_R {
        LDO15RF_VOUT_SEL_AON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&self) -> LDO15RF_CC_AON_R {
        LDO15RF_CC_AON_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&self) -> LDO15RF_BYPASS_AON_R {
        LDO15RF_BYPASS_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn reserved_29_31(&self) -> RESERVED_29_31_R {
        RESERVED_29_31_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mbg_aon(&mut self) -> PU_MBG_AON_W<0> {
        PU_MBG_AON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo15rf_aon(&mut self) -> PU_LDO15RF_AON_W<1> {
        PU_LDO15RF_AON_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_sfreg_aon(&mut self) -> PU_SFREG_AON_W<2> {
        PU_SFREG_AON_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal_buf_aon(&mut self) -> PU_XTAL_BUF_AON_W<4> {
        PU_XTAL_BUF_AON_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal_aon(&mut self) -> PU_XTAL_AON_W<5> {
        PU_XTAL_AON_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_sstart_sel_aon(&mut self) -> LDO15RF_SSTART_SEL_AON_W<8> {
        LDO15RF_SSTART_SEL_AON_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_sstart_delay_aon(&mut self) -> LDO15RF_SSTART_DELAY_AON_W<9> {
        LDO15RF_SSTART_DELAY_AON_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_pulldown_aon(&mut self) -> LDO15RF_PULLDOWN_AON_W<12> {
        LDO15RF_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_pulldown_sel_aon(&mut self) -> LDO15RF_PULLDOWN_SEL_AON_W<13> {
        LDO15RF_PULLDOWN_SEL_AON_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_vout_sel_aon(&mut self) -> LDO15RF_VOUT_SEL_AON_W<16> {
        LDO15RF_VOUT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_cc_aon(&mut self) -> LDO15RF_CC_AON_W<24> {
        LDO15RF_CC_AON_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_bypass_aon(&mut self) -> LDO15RF_BYPASS_AON_W<28> {
        LDO15RF_BYPASS_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_top_aon\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_top_aon](index.html) module"]
pub struct RF_TOP_AON_SPEC;
impl crate::RegisterSpec for RF_TOP_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_top_aon::R](R) reader structure"]
impl crate::Readable for RF_TOP_AON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_top_aon::W](W) writer structure"]
impl crate::Writable for RF_TOP_AON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_top_aon to value 0x0002_0137"]
impl crate::Resettable for RF_TOP_AON_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0137;
}
