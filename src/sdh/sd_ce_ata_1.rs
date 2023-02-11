#[doc = "Register `sd_ce_ata_1` reader"]
pub struct R(crate::R<SD_CE_ATA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CE_ATA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CE_ATA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CE_ATA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_ce_ata_1` writer"]
pub struct W(crate::W<SD_CE_ATA_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CE_ATA_1_SPEC>;
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
impl From<crate::W<SD_CE_ATA_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CE_ATA_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpl_timeout` reader - "]
pub type CPL_TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cpl_timeout` writer - "]
pub type CPL_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_CE_ATA_1_SPEC, u16, u16, 14, O>;
#[doc = "Field `reserved_15_14` reader - "]
pub type RESERVED_15_14_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn cpl_timeout(&self) -> CPL_TIMEOUT_R {
        CPL_TIMEOUT_R::new(self.bits & 0x3fff)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_15_14(&self) -> RESERVED_15_14_R {
        RESERVED_15_14_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn cpl_timeout(&mut self) -> CPL_TIMEOUT_W<0> {
        CPL_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CE-ATA Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_ce_ata_1](index.html) module"]
pub struct SD_CE_ATA_1_SPEC;
impl crate::RegisterSpec for SD_CE_ATA_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_ce_ata_1::R](R) reader structure"]
impl crate::Readable for SD_CE_ATA_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_ce_ata_1::W](W) writer structure"]
impl crate::Writable for SD_CE_ATA_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_ce_ata_1 to value 0x3fff"]
impl crate::Resettable for SD_CE_ATA_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
