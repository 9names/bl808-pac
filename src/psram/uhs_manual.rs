#[doc = "Register `uhs_manual` reader"]
pub struct R(crate::R<UHS_MANUAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_MANUAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_MANUAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_MANUAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_manual` writer"]
pub struct W(crate::W<UHS_MANUAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_MANUAL_SPEC>;
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
impl From<crate::W<UHS_MANUAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_MANUAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_force_ceb_low` reader - "]
pub type REG_FORCE_CEB_LOW_R = crate::BitReader<bool>;
#[doc = "Field `reg_force_ceb_low` writer - "]
pub type REG_FORCE_CEB_LOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_MANUAL_SPEC, bool, O>;
#[doc = "Field `reg_force_ceb_high` reader - "]
pub type REG_FORCE_CEB_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `reg_force_ceb_high` writer - "]
pub type REG_FORCE_CEB_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UHS_MANUAL_SPEC, bool, O>;
#[doc = "Field `reg_psram_resetb` reader - "]
pub type REG_PSRAM_RESETB_R = crate::BitReader<bool>;
#[doc = "Field `reg_psram_resetb` writer - "]
pub type REG_PSRAM_RESETB_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_MANUAL_SPEC, bool, O>;
#[doc = "Field `reg_x16_mode` reader - "]
pub type REG_X16_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_x16_mode` writer - "]
pub type REG_X16_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_MANUAL_SPEC, bool, O>;
#[doc = "Field `reg_wrap2incr_en` reader - "]
pub type REG_WRAP2INCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_wrap2incr_en` writer - "]
pub type REG_WRAP2INCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_MANUAL_SPEC, bool, O>;
#[doc = "Field `reserved_5_15` reader - "]
pub type RESERVED_5_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_pck_s_div` reader - "]
pub type REG_PCK_S_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pck_s_div` writer - "]
pub type REG_PCK_S_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_MANUAL_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pck_t_div` reader - "]
pub type REG_PCK_T_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pck_t_div` writer - "]
pub type REG_PCK_T_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_MANUAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&self) -> REG_FORCE_CEB_LOW_R {
        REG_FORCE_CEB_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&self) -> REG_FORCE_CEB_HIGH_R {
        REG_FORCE_CEB_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_psram_resetb(&self) -> REG_PSRAM_RESETB_R {
        REG_PSRAM_RESETB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_x16_mode(&self) -> REG_X16_MODE_R {
        REG_X16_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&self) -> REG_WRAP2INCR_EN_R {
        REG_WRAP2INCR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn reserved_5_15(&self) -> RESERVED_5_15_R {
        RESERVED_5_15_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn reg_pck_s_div(&self) -> REG_PCK_S_DIV_R {
        REG_PCK_S_DIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_pck_t_div(&self) -> REG_PCK_T_DIV_R {
        REG_PCK_T_DIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_force_ceb_low(&mut self) -> REG_FORCE_CEB_LOW_W<0> {
        REG_FORCE_CEB_LOW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_force_ceb_high(&mut self) -> REG_FORCE_CEB_HIGH_W<1> {
        REG_FORCE_CEB_HIGH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_resetb(&mut self) -> REG_PSRAM_RESETB_W<2> {
        REG_PSRAM_RESETB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_x16_mode(&mut self) -> REG_X16_MODE_W<3> {
        REG_X16_MODE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wrap2incr_en(&mut self) -> REG_WRAP2INCR_EN_W<4> {
        REG_WRAP2INCR_EN_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pck_s_div(&mut self) -> REG_PCK_S_DIV_W<16> {
        REG_PCK_S_DIV_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pck_t_div(&mut self) -> REG_PCK_T_DIV_W<24> {
        REG_PCK_T_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_manual\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_manual](index.html) module"]
pub struct UHS_MANUAL_SPEC;
impl crate::RegisterSpec for UHS_MANUAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_manual::R](R) reader structure"]
impl crate::Readable for UHS_MANUAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_manual::W](W) writer structure"]
impl crate::Writable for UHS_MANUAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_manual to value 0x4000_001c"]
impl crate::Resettable for UHS_MANUAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_001c;
}
