#[doc = "Register `rc32k_ctrl0` reader"]
pub struct R(crate::R<RC32K_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32K_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32K_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32K_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32k_ctrl0` writer"]
pub struct W(crate::W<RC32K_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32K_CTRL0_SPEC>;
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
impl From<crate::W<RC32K_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32K_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_ctl` reader - "]
pub type RTC_CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_ctl` writer - "]
pub type RTC_CTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC32K_CTRL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `rtc_dly_option` reader - "]
pub type RTC_DLY_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `rtc_dly_option` writer - "]
pub type RTC_DLY_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `pu_ldo18io_aon` reader - "]
pub type PU_LDO18IO_AON_R = crate::BitReader<bool>;
#[doc = "Field `pu_ldo18io_aon` writer - "]
pub type PU_LDO18IO_AON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `hbn_mode` writer - "]
pub type HBN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `trap_mode` reader - "]
pub type TRAP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `pwrdn_hbn_core` reader - "]
pub type PWRDN_HBN_CORE_R = crate::BitReader<bool>;
#[doc = "Field `pwrdn_hbn_core` writer - "]
pub type PWRDN_HBN_CORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `reserved_10_11` reader - "]
pub type RESERVED_10_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_rst` reader - "]
pub type SW_RST_R = crate::BitReader<bool>;
#[doc = "Field `sw_rst` writer - "]
pub type SW_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` reader - "]
pub type HBN_DIS_PWR_OFF_LDO11_R = crate::BitReader<bool>;
#[doc = "Field `hbn_dis_pwr_off_ldo11` writer - "]
pub type HBN_DIS_PWR_OFF_LDO11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RTC_CTL_R {
        RTC_CTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RTC_DLY_OPTION_R {
        RTC_DLY_OPTION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_ldo18io_aon(&self) -> PU_LDO18IO_AON_R {
        PU_LDO18IO_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TRAP_MODE_R {
        TRAP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PWRDN_HBN_CORE_R {
        PWRDN_HBN_CORE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reserved_10_11(&self) -> RESERVED_10_11_R {
        RESERVED_10_11_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HBN_DIS_PWR_OFF_LDO11_R {
        HBN_DIS_PWR_OFF_LDO11_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctl(&mut self) -> RTC_CTL_W<0> {
        RTC_CTL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_dly_option(&mut self) -> RTC_DLY_OPTION_W<4> {
        RTC_DLY_OPTION_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo18io_aon(&mut self) -> PU_LDO18IO_AON_W<5> {
        PU_LDO18IO_AON_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_mode(&mut self) -> HBN_MODE_W<7> {
        HBN_MODE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_hbn_core(&mut self) -> PWRDN_HBN_CORE_W<9> {
        PWRDN_HBN_CORE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst(&mut self) -> SW_RST_W<12> {
        SW_RST_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HBN_DIS_PWR_OFF_LDO11_W<13> {
        HBN_DIS_PWR_OFF_LDO11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_CTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32k_ctrl0](index.html) module"]
pub struct RC32K_CTRL0_SPEC;
impl crate::RegisterSpec for RC32K_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32k_ctrl0::R](R) reader structure"]
impl crate::Readable for RC32K_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32k_ctrl0::W](W) writer structure"]
impl crate::Writable for RC32K_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32k_ctrl0 to value 0x20"]
impl crate::Resettable for RC32K_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
