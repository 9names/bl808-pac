#[doc = "Register `sd_ce_ata_2` reader"]
pub struct R(crate::R<SD_CE_ATA_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CE_ATA_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CE_ATA_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CE_ATA_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_ce_ata_2` writer"]
pub struct W(crate::W<SD_CE_ATA_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CE_ATA_2_SPEC>;
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
impl From<crate::W<SD_CE_ATA_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CE_ATA_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `misc_int_int_en` reader - "]
pub type MISC_INT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `misc_int_int_en` writer - "]
pub type MISC_INT_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `misc_int_en` reader - "]
pub type MISC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `misc_int_en` writer - "]
pub type MISC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `misc_int` reader - "]
pub type MISC_INT_R = crate::BitReader<bool>;
#[doc = "Field `misc_int` writer - "]
pub type MISC_INT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `cpl_complete_int_en` reader - "]
pub type CPL_COMPLETE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpl_complete_int_en` writer - "]
pub type CPL_COMPLETE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `cpl_complete_en` reader - "]
pub type CPL_COMPLETE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cpl_complete_en` writer - "]
pub type CPL_COMPLETE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `cpl_complete` reader - "]
pub type CPL_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `cpl_complete` writer - "]
pub type CPL_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `reserved_10_7` reader - "]
pub type RESERVED_10_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mmc_resetn` reader - "]
pub type MMC_RESETN_R = crate::BitReader<bool>;
#[doc = "Field `mmc_resetn` writer - "]
pub type MMC_RESETN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `mmc_card` reader - "]
pub type MMC_CARD_R = crate::BitReader<bool>;
#[doc = "Field `mmc_card` writer - "]
pub type MMC_CARD_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `ceata_card` reader - "]
pub type CEATA_CARD_R = crate::BitReader<bool>;
#[doc = "Field `ceata_card` writer - "]
pub type CEATA_CARD_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `snd_cpl` reader - "]
pub type SND_CPL_R = crate::BitReader<bool>;
#[doc = "Field `snd_cpl` writer - "]
pub type SND_CPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
#[doc = "Field `chk_cpl` reader - "]
pub type CHK_CPL_R = crate::BitReader<bool>;
#[doc = "Field `chk_cpl` writer - "]
pub type CHK_CPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_CE_ATA_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn misc_int_int_en(&self) -> MISC_INT_INT_EN_R {
        MISC_INT_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn misc_int_en(&self) -> MISC_INT_EN_R {
        MISC_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn misc_int(&self) -> MISC_INT_R {
        MISC_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cpl_complete_int_en(&self) -> CPL_COMPLETE_INT_EN_R {
        CPL_COMPLETE_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpl_complete_en(&self) -> CPL_COMPLETE_EN_R {
        CPL_COMPLETE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpl_complete(&self) -> CPL_COMPLETE_R {
        CPL_COMPLETE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10"]
    #[inline(always)]
    pub fn reserved_10_7(&self) -> RESERVED_10_7_R {
        RESERVED_10_7_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mmc_resetn(&self) -> MMC_RESETN_R {
        MMC_RESETN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mmc_card(&self) -> MMC_CARD_R {
        MMC_CARD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ceata_card(&self) -> CEATA_CARD_R {
        CEATA_CARD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn snd_cpl(&self) -> SND_CPL_R {
        SND_CPL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chk_cpl(&self) -> CHK_CPL_R {
        CHK_CPL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn misc_int_int_en(&mut self) -> MISC_INT_INT_EN_W<0> {
        MISC_INT_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn misc_int_en(&mut self) -> MISC_INT_EN_W<1> {
        MISC_INT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn misc_int(&mut self) -> MISC_INT_W<2> {
        MISC_INT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cpl_complete_int_en(&mut self) -> CPL_COMPLETE_INT_EN_W<4> {
        CPL_COMPLETE_INT_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cpl_complete_en(&mut self) -> CPL_COMPLETE_EN_W<5> {
        CPL_COMPLETE_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cpl_complete(&mut self) -> CPL_COMPLETE_W<6> {
        CPL_COMPLETE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn mmc_resetn(&mut self) -> MMC_RESETN_W<11> {
        MMC_RESETN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn mmc_card(&mut self) -> MMC_CARD_W<12> {
        MMC_CARD_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_card(&mut self) -> CEATA_CARD_W<13> {
        CEATA_CARD_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn snd_cpl(&mut self) -> SND_CPL_W<14> {
        SND_CPL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn chk_cpl(&mut self) -> CHK_CPL_W<15> {
        CHK_CPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CE-ATA Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_ce_ata_2](index.html) module"]
pub struct SD_CE_ATA_2_SPEC;
impl crate::RegisterSpec for SD_CE_ATA_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_ce_ata_2::R](R) reader structure"]
impl crate::Readable for SD_CE_ATA_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_ce_ata_2::W](W) writer structure"]
impl crate::Writable for SD_CE_ATA_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_ce_ata_2 to value 0x0800"]
impl crate::Resettable for SD_CE_ATA_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
