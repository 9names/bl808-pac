#[doc = "Register `pds_stat` reader"]
pub struct R(crate::R<PDS_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_stat` writer"]
pub struct W(crate::W<PDS_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_STAT_SPEC>;
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
impl From<crate::W<PDS_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pds_start_ps` writer - "]
pub type PDS_START_PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
#[doc = "Field `cr_sleep_forever` reader - "]
pub type CR_SLEEP_FOREVER_R = crate::BitReader<bool>;
#[doc = "Field `cr_sleep_forever` writer - "]
pub type CR_SLEEP_FOREVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
#[doc = "Field `cr_xtal_force_off` reader - "]
pub type CR_XTAL_FORCE_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_xtal_force_off` writer - "]
pub type CR_XTAL_FORCE_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
#[doc = "Field `cr_pds_wifi_save_state` reader - "]
pub type CR_PDS_WIFI_SAVE_STATE_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wifi_save_state` writer - "]
pub type CR_PDS_WIFI_SAVE_STATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_dcdc11` reader - "]
pub type CR_PDS_PD_DCDC11_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_dcdc11` writer - "]
pub type CR_PDS_PD_DCDC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
#[doc = "Field `cr_pds_pd_bg_sys` reader - "]
pub type CR_PDS_PD_BG_SYS_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pd_bg_sys` writer - "]
pub type CR_PDS_PD_BG_SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_sleep_forever(&self) -> CR_SLEEP_FOREVER_R {
        CR_SLEEP_FOREVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xtal_force_off(&self) -> CR_XTAL_FORCE_OFF_R {
        CR_XTAL_FORCE_OFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_wifi_save_state(&self) -> CR_PDS_WIFI_SAVE_STATE_R {
        CR_PDS_WIFI_SAVE_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_pd_dcdc11(&self) -> CR_PDS_PD_DCDC11_R {
        CR_PDS_PD_DCDC11_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_pd_bg_sys(&self) -> CR_PDS_PD_BG_SYS_R {
        CR_PDS_PD_BG_SYS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pds_start_ps(&mut self) -> PDS_START_PS_W<0> {
        PDS_START_PS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sleep_forever(&mut self) -> CR_SLEEP_FOREVER_W<1> {
        CR_SLEEP_FOREVER_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_xtal_force_off(&mut self) -> CR_XTAL_FORCE_OFF_W<2> {
        CR_XTAL_FORCE_OFF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_save_state(&mut self) -> CR_PDS_WIFI_SAVE_STATE_W<3> {
        CR_PDS_WIFI_SAVE_STATE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_dcdc11(&mut self) -> CR_PDS_PD_DCDC11_W<4> {
        CR_PDS_PD_DCDC11_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pd_bg_sys(&mut self) -> CR_PDS_PD_BG_SYS_W<5> {
        CR_PDS_PD_BG_SYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_stat](index.html) module"]
pub struct PDS_STAT_SPEC;
impl crate::RegisterSpec for PDS_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_stat::R](R) reader structure"]
impl crate::Readable for PDS_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_stat::W](W) writer structure"]
impl crate::Writable for PDS_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PDS_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
