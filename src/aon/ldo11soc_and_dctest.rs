#[doc = "Register `ldo11soc_and_dctest` reader"]
pub struct R(crate::R<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO11SOC_AND_DCTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO11SOC_AND_DCTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo11soc_and_dctest` writer"]
pub struct W(crate::W<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO11SOC_AND_DCTEST_SPEC>;
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
impl From<crate::W<LDO11SOC_AND_DCTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_3` reader - "]
pub type RESERVED_0_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_cfb_sel_aon` reader - "]
pub type DCDC11_CFB_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_cfb_sel_aon` writer - "]
pub type DCDC11_CFB_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc11_chf_sel_aon` reader - "]
pub type DCDC11_CHF_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_chf_sel_aon` writer - "]
pub type DCDC11_CHF_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 4, O>;
#[doc = "Field `dcdc11_comp_gm_sel_aon` reader - "]
pub type DCDC11_COMP_GM_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_comp_gm_sel_aon` writer - "]
pub type DCDC11_COMP_GM_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_cs_delay_aon` reader - "]
pub type DCDC11_CS_DELAY_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_cs_delay_aon` writer - "]
pub type DCDC11_CS_DELAY_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_drv_sr_aon` reader - "]
pub type DCDC11_DRV_SR_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_drv_sr_aon` writer - "]
pub type DCDC11_DRV_SR_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `dcdc11_en_antiring_aon` reader - "]
pub type DCDC11_EN_ANTIRING_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_en_antiring_aon` writer - "]
pub type DCDC11_EN_ANTIRING_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_en_osc_inhibit_t2_aon` reader - "]
pub type DCDC11_EN_OSC_INHIBIT_T2_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_en_osc_inhibit_t2_aon` writer - "]
pub type DCDC11_EN_OSC_INHIBIT_T2_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_en_slow_osc_aon` reader - "]
pub type DCDC11_EN_SLOW_OSC_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_en_slow_osc_aon` writer - "]
pub type DCDC11_EN_SLOW_OSC_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_en_stby_lp_aon` reader - "]
pub type DCDC11_EN_STBY_LP_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_en_stby_lp_aon` writer - "]
pub type DCDC11_EN_STBY_LP_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_en_stop_osc_aon` reader - "]
pub type DCDC11_EN_STOP_OSC_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_en_stop_osc_aon` writer - "]
pub type DCDC11_EN_STOP_OSC_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_force_en_cs_zvs_aon` reader - "]
pub type DCDC11_FORCE_EN_CS_ZVS_AON_R = crate::BitReader<bool>;
#[doc = "Field `dcdc11_force_en_cs_zvs_aon` writer - "]
pub type DCDC11_FORCE_EN_CS_ZVS_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, bool, O>;
#[doc = "Field `dcdc11_isense_trim_aon` reader - "]
pub type DCDC11_ISENSE_TRIM_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dcdc11_isense_trim_aon` writer - "]
pub type DCDC11_ISENSE_TRIM_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDO11SOC_AND_DCTEST_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reserved_0_3(&self) -> RESERVED_0_3_R {
        RESERVED_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dcdc11_cfb_sel_aon(&self) -> DCDC11_CFB_SEL_AON_R {
        DCDC11_CFB_SEL_AON_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc11_chf_sel_aon(&self) -> DCDC11_CHF_SEL_AON_R {
        DCDC11_CHF_SEL_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc11_comp_gm_sel_aon(&self) -> DCDC11_COMP_GM_SEL_AON_R {
        DCDC11_COMP_GM_SEL_AON_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc11_cs_delay_aon(&self) -> DCDC11_CS_DELAY_AON_R {
        DCDC11_CS_DELAY_AON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dcdc11_drv_sr_aon(&self) -> DCDC11_DRV_SR_AON_R {
        DCDC11_DRV_SR_AON_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dcdc11_en_antiring_aon(&self) -> DCDC11_EN_ANTIRING_AON_R {
        DCDC11_EN_ANTIRING_AON_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dcdc11_en_osc_inhibit_t2_aon(&self) -> DCDC11_EN_OSC_INHIBIT_T2_AON_R {
        DCDC11_EN_OSC_INHIBIT_T2_AON_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dcdc11_en_slow_osc_aon(&self) -> DCDC11_EN_SLOW_OSC_AON_R {
        DCDC11_EN_SLOW_OSC_AON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc11_en_stby_lp_aon(&self) -> DCDC11_EN_STBY_LP_AON_R {
        DCDC11_EN_STBY_LP_AON_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc11_en_stop_osc_aon(&self) -> DCDC11_EN_STOP_OSC_AON_R {
        DCDC11_EN_STOP_OSC_AON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc11_force_en_cs_zvs_aon(&self) -> DCDC11_FORCE_EN_CS_ZVS_AON_R {
        DCDC11_FORCE_EN_CS_ZVS_AON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn dcdc11_isense_trim_aon(&self) -> DCDC11_ISENSE_TRIM_AON_R {
        DCDC11_ISENSE_TRIM_AON_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_cfb_sel_aon(&mut self) -> DCDC11_CFB_SEL_AON_W<4> {
        DCDC11_CFB_SEL_AON_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_chf_sel_aon(&mut self) -> DCDC11_CHF_SEL_AON_W<8> {
        DCDC11_CHF_SEL_AON_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_comp_gm_sel_aon(&mut self) -> DCDC11_COMP_GM_SEL_AON_W<12> {
        DCDC11_COMP_GM_SEL_AON_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_cs_delay_aon(&mut self) -> DCDC11_CS_DELAY_AON_W<16> {
        DCDC11_CS_DELAY_AON_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_drv_sr_aon(&mut self) -> DCDC11_DRV_SR_AON_W<20> {
        DCDC11_DRV_SR_AON_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_en_antiring_aon(&mut self) -> DCDC11_EN_ANTIRING_AON_W<22> {
        DCDC11_EN_ANTIRING_AON_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_en_osc_inhibit_t2_aon(&mut self) -> DCDC11_EN_OSC_INHIBIT_T2_AON_W<23> {
        DCDC11_EN_OSC_INHIBIT_T2_AON_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_en_slow_osc_aon(&mut self) -> DCDC11_EN_SLOW_OSC_AON_W<24> {
        DCDC11_EN_SLOW_OSC_AON_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_en_stby_lp_aon(&mut self) -> DCDC11_EN_STBY_LP_AON_W<25> {
        DCDC11_EN_STBY_LP_AON_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_en_stop_osc_aon(&mut self) -> DCDC11_EN_STOP_OSC_AON_W<26> {
        DCDC11_EN_STOP_OSC_AON_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_force_en_cs_zvs_aon(&mut self) -> DCDC11_FORCE_EN_CS_ZVS_AON_W<27> {
        DCDC11_FORCE_EN_CS_ZVS_AON_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc11_isense_trim_aon(&mut self) -> DCDC11_ISENSE_TRIM_AON_W<28> {
        DCDC11_ISENSE_TRIM_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo11soc_and_dctest\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo11soc_and_dctest](index.html) module"]
pub struct LDO11SOC_AND_DCTEST_SPEC;
impl crate::RegisterSpec for LDO11SOC_AND_DCTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo11soc_and_dctest::R](R) reader structure"]
impl crate::Readable for LDO11SOC_AND_DCTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo11soc_and_dctest::W](W) writer structure"]
impl crate::Writable for LDO11SOC_AND_DCTEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo11soc_and_dctest to value 0x46f4_4180"]
impl crate::Resettable for LDO11SOC_AND_DCTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x46f4_4180;
}
