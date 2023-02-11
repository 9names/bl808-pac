#[doc = "Register `pds_int` reader"]
pub struct R(crate::R<PDS_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_int` writer"]
pub struct W(crate::W<PDS_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_INT_SPEC>;
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
impl From<crate::W<PDS_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ro_pds_wake_int` reader - "]
pub type RO_PDS_WAKE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub type RO_PDS_RF_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_wifi_tbtt_sleep_irq` reader - "]
pub type RO_PDS_WIFI_TBTT_SLEEP_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_wifi_tbtt_wakeup_irq` reader - "]
pub type RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub type CR_PDS_WAKE_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub type CR_PDS_WAKE_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub type CR_PDS_RF_DONE_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub type CR_PDS_RF_DONE_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` reader - "]
pub type CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wifi_tbtt_sleep_irq_mask` writer - "]
pub type CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` reader - "]
pub type CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wifi_tbtt_wakeup_irq_mask` writer - "]
pub type CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_int_clr` reader - "]
pub type CR_PDS_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_int_clr` writer - "]
pub type CR_PDS_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `reserved_9` reader - "]
pub type RESERVED_9_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wakeup_src_en` reader - "]
pub type CR_PDS_WAKEUP_SRC_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_pds_wakeup_src_en` writer - "]
pub type CR_PDS_WAKEUP_SRC_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_INT_SPEC, u16, u16, 11, O>;
#[doc = "Field `ro_pds_wakeup_event` reader - "]
pub type RO_PDS_WAKEUP_EVENT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_sleep_irq(&self) -> RO_PDS_WIFI_TBTT_SLEEP_IRQ_R {
        RO_PDS_WIFI_TBTT_SLEEP_IRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_wifi_tbtt_wakeup_irq(&self) -> RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R {
        RO_PDS_WIFI_TBTT_WAKEUP_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(&self) -> CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R {
        CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(&self) -> CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R {
        CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reserved_9(&self) -> RESERVED_9_R {
        RESERVED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:20"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&self) -> CR_PDS_WAKEUP_SRC_EN_R {
        CR_PDS_WAKEUP_SRC_EN_R::new(((self.bits >> 10) & 0x07ff) as u16)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&self) -> RO_PDS_WAKEUP_EVENT_R {
        RO_PDS_WAKEUP_EVENT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W<4> {
        CR_PDS_WAKE_INT_MASK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W<5> {
        CR_PDS_RF_DONE_INT_MASK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_tbtt_sleep_irq_mask(&mut self) -> CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W<6> {
        CR_PDS_WIFI_TBTT_SLEEP_IRQ_MASK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wifi_tbtt_wakeup_irq_mask(&mut self) -> CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W<7> {
        CR_PDS_WIFI_TBTT_WAKEUP_IRQ_MASK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W<8> {
        CR_PDS_INT_CLR_W::new(self)
    }
    #[doc = "Bits 10:20"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_wakeup_src_en(&mut self) -> CR_PDS_WAKEUP_SRC_EN_W<10> {
        CR_PDS_WAKEUP_SRC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_INT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_int](index.html) module"]
pub struct PDS_INT_SPEC;
impl crate::RegisterSpec for PDS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_int::R](R) reader structure"]
impl crate::Readable for PDS_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_int::W](W) writer structure"]
impl crate::Writable for PDS_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_int to value 0x001f_fc00"]
impl crate::Resettable for PDS_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_fc00;
}
