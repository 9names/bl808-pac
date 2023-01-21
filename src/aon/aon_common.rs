#[doc = "Register `aon_common` reader"]
pub struct R(crate::R<AON_COMMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_COMMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AON_COMMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AON_COMMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon_common` writer"]
pub struct W(crate::W<AON_COMMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_COMMON_SPEC>;
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
impl From<crate::W<AON_COMMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AON_COMMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmux_aon` reader - "]
pub type TMUX_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tmux_aon` writer - "]
pub type TMUX_AON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AON_COMMON_SPEC, u8, u8, 3, O>;
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub type PMIP_DC_TP_OUT_EN_AON_R = crate::BitReader<bool>;
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub type PMIP_DC_TP_OUT_EN_AON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub type TEN_BG_SYS_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub type TEN_BG_SYS_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc11_0_aon` reader - "]
pub type TEN_DCDC11_0_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc11_0_aon` writer - "]
pub type TEN_DCDC11_0_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc11_1_aon` reader - "]
pub type TEN_DCDC11_1_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc11_1_aon` writer - "]
pub type TEN_DCDC11_1_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc18_0_aon` reader - "]
pub type TEN_DCDC18_0_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc18_0_aon` writer - "]
pub type TEN_DCDC18_0_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_dcdc18_1_aon` reader - "]
pub type TEN_DCDC18_1_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_dcdc18_1_aon` writer - "]
pub type TEN_DCDC18_1_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo12uhs` reader - "]
pub type TEN_LDO12UHS_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo12uhs` writer - "]
pub type TEN_LDO12UHS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo18flash` reader - "]
pub type TEN_LDO18FLASH_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo18flash` writer - "]
pub type TEN_LDO18FLASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo15cis` reader - "]
pub type TEN_LDO15CIS_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo15cis` writer - "]
pub type TEN_LDO15CIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo18io_aon` reader - "]
pub type TEN_LDO18IO_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo18io_aon` writer - "]
pub type TEN_LDO18IO_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_ldo28cis` reader - "]
pub type TEN_LDO28CIS_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo28cis` writer - "]
pub type TEN_LDO28CIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_rc32m` reader - "]
pub type TEN_RC32M_R = crate::BitReader<bool>;
#[doc = "Field `ten_rc32m` writer - "]
pub type TEN_RC32M_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub type TEN_LDO15RF_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub type TEN_LDO15RF_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_xtal_aon` reader - "]
pub type TEN_XTAL_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_xtal_aon` writer - "]
pub type TEN_XTAL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `dten_xtal_aon` reader - "]
pub type DTEN_XTAL_AON_R = crate::BitReader<bool>;
#[doc = "Field `dten_xtal_aon` writer - "]
pub type DTEN_XTAL_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_mbg_aon` reader - "]
pub type TEN_MBG_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_mbg_aon` writer - "]
pub type TEN_MBG_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub type TEN_CIP_MISC_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub type TEN_CIP_MISC_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `ten_aon` reader - "]
pub type TEN_AON_R = crate::BitReader<bool>;
#[doc = "Field `ten_aon` writer - "]
pub type TEN_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_COMMON_SPEC, bool, O>;
#[doc = "Field `reserved_22_31` reader - "]
pub type RESERVED_22_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TMUX_AON_R {
        TMUX_AON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PMIP_DC_TP_OUT_EN_AON_R {
        PMIP_DC_TP_OUT_EN_AON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TEN_BG_SYS_AON_R {
        TEN_BG_SYS_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ten_dcdc11_0_aon(&self) -> TEN_DCDC11_0_AON_R {
        TEN_DCDC11_0_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_dcdc11_1_aon(&self) -> TEN_DCDC11_1_AON_R {
        TEN_DCDC11_1_AON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TEN_DCDC18_0_AON_R {
        TEN_DCDC18_0_AON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TEN_DCDC18_1_AON_R {
        TEN_DCDC18_1_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo12uhs(&self) -> TEN_LDO12UHS_R {
        TEN_LDO12UHS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_ldo18flash(&self) -> TEN_LDO18FLASH_R {
        TEN_LDO18FLASH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_ldo15cis(&self) -> TEN_LDO15CIS_R {
        TEN_LDO15CIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_ldo18io_aon(&self) -> TEN_LDO18IO_AON_R {
        TEN_LDO18IO_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_ldo28cis(&self) -> TEN_LDO28CIS_R {
        TEN_LDO28CIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_rc32m(&self) -> TEN_RC32M_R {
        TEN_RC32M_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TEN_LDO15RF_AON_R {
        TEN_LDO15RF_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TEN_XTAL_AON_R {
        TEN_XTAL_AON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DTEN_XTAL_AON_R {
        DTEN_XTAL_AON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TEN_MBG_AON_R {
        TEN_MBG_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TEN_CIP_MISC_AON_R {
        TEN_CIP_MISC_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TEN_AON_R {
        TEN_AON_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn reserved_22_31(&self) -> RESERVED_22_31_R {
        RESERVED_22_31_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_aon(&mut self) -> TMUX_AON_W<0> {
        TMUX_AON_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PMIP_DC_TP_OUT_EN_AON_W<3> {
        PMIP_DC_TP_OUT_EN_AON_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ten_bg_sys_aon(&mut self) -> TEN_BG_SYS_AON_W<4> {
        TEN_BG_SYS_AON_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc11_0_aon(&mut self) -> TEN_DCDC11_0_AON_W<5> {
        TEN_DCDC11_0_AON_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc11_1_aon(&mut self) -> TEN_DCDC11_1_AON_W<6> {
        TEN_DCDC11_1_AON_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc18_0_aon(&mut self) -> TEN_DCDC18_0_AON_W<7> {
        TEN_DCDC18_0_AON_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc18_1_aon(&mut self) -> TEN_DCDC18_1_AON_W<8> {
        TEN_DCDC18_1_AON_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo12uhs(&mut self) -> TEN_LDO12UHS_W<9> {
        TEN_LDO12UHS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo18flash(&mut self) -> TEN_LDO18FLASH_W<10> {
        TEN_LDO18FLASH_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo15cis(&mut self) -> TEN_LDO15CIS_W<11> {
        TEN_LDO15CIS_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo18io_aon(&mut self) -> TEN_LDO18IO_AON_W<12> {
        TEN_LDO18IO_AON_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo28cis(&mut self) -> TEN_LDO28CIS_W<13> {
        TEN_LDO28CIS_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ten_rc32m(&mut self) -> TEN_RC32M_W<14> {
        TEN_RC32M_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo15rf_aon(&mut self) -> TEN_LDO15RF_AON_W<16> {
        TEN_LDO15RF_AON_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ten_xtal_aon(&mut self) -> TEN_XTAL_AON_W<17> {
        TEN_XTAL_AON_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn dten_xtal_aon(&mut self) -> DTEN_XTAL_AON_W<18> {
        DTEN_XTAL_AON_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ten_mbg_aon(&mut self) -> TEN_MBG_AON_W<19> {
        TEN_MBG_AON_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ten_cip_misc_aon(&mut self) -> TEN_CIP_MISC_AON_W<20> {
        TEN_CIP_MISC_AON_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ten_aon(&mut self) -> TEN_AON_W<21> {
        TEN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon_common\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_common](index.html) module"]
pub struct AON_COMMON_SPEC;
impl crate::RegisterSpec for AON_COMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon_common::R](R) reader structure"]
impl crate::Readable for AON_COMMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon_common::W](W) writer structure"]
impl crate::Writable for AON_COMMON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_common to value 0"]
impl crate::Resettable for AON_COMMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
