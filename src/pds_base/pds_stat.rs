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
#[doc = "Field `ro_pds_state` reader - "]
pub type RO_PDS_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ro_pds_rf_state` reader - "]
pub type RO_PDS_RF_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_13_23` reader - "]
pub type RESERVED_13_23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pds_reset_event` reader - "]
pub type PDS_RESET_EVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_27_30` reader - "]
pub type RESERVED_27_30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_clr_reset_event` writer - "]
pub type PDS_CLR_RESET_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RO_PDS_STATE_R {
        RO_PDS_STATE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RO_PDS_RF_STATE_R {
        RO_PDS_RF_STATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:23"]
    #[inline(always)]
    pub fn reserved_13_23(&self) -> RESERVED_13_23_R {
        RESERVED_13_23_R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pds_reset_event(&self) -> PDS_RESET_EVENT_R {
        PDS_RESET_EVENT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:30"]
    #[inline(always)]
    pub fn reserved_27_30(&self) -> RESERVED_27_30_R {
        RESERVED_27_30_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pds_clr_reset_event(&mut self) -> PDS_CLR_RESET_EVENT_W<31> {
        PDS_CLR_RESET_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_stat\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_stat](index.html) module"]
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
