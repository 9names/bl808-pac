#[doc = "Register `sd_present_state_2` reader"]
pub struct R(crate::R<SD_PRESENT_STATE_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_PRESENT_STATE_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_PRESENT_STATE_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_PRESENT_STATE_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_present_state_2` writer"]
pub struct W(crate::W<SD_PRESENT_STATE_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_PRESENT_STATE_2_SPEC>;
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
impl From<crate::W<SD_PRESENT_STATE_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_PRESENT_STATE_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `card_inserted` reader - "]
pub type CARD_INSERTED_R = crate::BitReader<bool>;
#[doc = "Field `card_stable` reader - "]
pub type CARD_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `card_det` reader - "]
pub type CARD_DET_R = crate::BitReader<bool>;
#[doc = "Field `write_prot` reader - "]
pub type WRITE_PROT_R = crate::BitReader<bool>;
#[doc = "Field `dat_level` reader - "]
pub type DAT_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_level` reader - "]
pub type CMD_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `reserved_15_9` reader - "]
pub type RESERVED_15_9_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CARD_INSERTED_R {
        CARD_INSERTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn card_stable(&self) -> CARD_STABLE_R {
        CARD_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn card_det(&self) -> CARD_DET_R {
        CARD_DET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn write_prot(&self) -> WRITE_PROT_R {
        WRITE_PROT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn dat_level(&self) -> DAT_LEVEL_R {
        DAT_LEVEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cmd_level(&self) -> CMD_LEVEL_R {
        CMD_LEVEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn reserved_15_9(&self) -> RESERVED_15_9_R {
        RESERVED_15_9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Present State Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_present_state_2](index.html) module"]
pub struct SD_PRESENT_STATE_2_SPEC;
impl crate::RegisterSpec for SD_PRESENT_STATE_2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_present_state_2::R](R) reader structure"]
impl crate::Readable for SD_PRESENT_STATE_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_present_state_2::W](W) writer structure"]
impl crate::Writable for SD_PRESENT_STATE_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_present_state_2 to value 0x01f0"]
impl crate::Resettable for SD_PRESENT_STATE_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01f0;
}
