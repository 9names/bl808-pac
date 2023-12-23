#[doc = "Register `sd_slot_int_status` reader"]
pub struct R(crate::R<SD_SLOT_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SLOT_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SLOT_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SLOT_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_slot_int_status` writer"]
pub struct W(crate::W<SD_SLOT_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SLOT_INT_STATUS_SPEC>;
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
impl From<crate::W<SD_SLOT_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SLOT_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slot_int0` reader - "]
pub type SLOT_INT0_R = crate::BitReader<bool>;
#[doc = "Field `slot_int1` reader - "]
pub type SLOT_INT1_R = crate::BitReader<bool>;
#[doc = "Field `reserved_15_2` reader - "]
pub type RESERVED_15_2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slot_int0(&self) -> SLOT_INT0_R {
        SLOT_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slot_int1(&self) -> SLOT_INT1_R {
        SLOT_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn reserved_15_2(&self) -> RESERVED_15_2_R {
        RESERVED_15_2_R::new((self.bits >> 2) & 0x3fff)
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
#[doc = "Slot Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_slot_int_status](index.html) module"]
pub struct SD_SLOT_INT_STATUS_SPEC;
impl crate::RegisterSpec for SD_SLOT_INT_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_slot_int_status::R](R) reader structure"]
impl crate::Readable for SD_SLOT_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_slot_int_status::W](W) writer structure"]
impl crate::Writable for SD_SLOT_INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_slot_int_status to value 0"]
impl crate::Resettable for SD_SLOT_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
