#[doc = "Register `proc_mon` reader"]
pub struct R(crate::R<PROC_MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC_MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC_MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC_MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `proc_mon` writer"]
pub struct W(crate::W<PROC_MON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC_MON_SPEC>;
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
impl From<crate::W<PROC_MON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC_MON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_proc_mon` reader - "]
pub type PU_PROC_MON_R = crate::BitReader<bool>;
#[doc = "Field `pu_proc_mon` writer - "]
pub type PU_PROC_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `osc_en_rvt` reader - "]
pub type OSC_EN_RVT_R = crate::BitReader<bool>;
#[doc = "Field `osc_en_rvt` writer - "]
pub type OSC_EN_RVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `osc_en_lvt` reader - "]
pub type OSC_EN_LVT_R = crate::BitReader<bool>;
#[doc = "Field `osc_en_lvt` writer - "]
pub type OSC_EN_LVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `osc_sel` reader - "]
pub type OSC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `osc_sel` writer - "]
pub type OSC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `rstn_ringcount` reader - "]
pub type RSTN_RINGCOUNT_R = crate::BitReader<bool>;
#[doc = "Field `rstn_ringcount` writer - "]
pub type RSTN_RINGCOUNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `rstn_refcount` reader - "]
pub type RSTN_REFCOUNT_R = crate::BitReader<bool>;
#[doc = "Field `rstn_refcount` writer - "]
pub type RSTN_REFCOUNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROC_MON_SPEC, bool, O>;
#[doc = "Field `reserved_6_7` reader - "]
pub type RESERVED_6_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `refcount_div_onehot` reader - "]
pub type REFCOUNT_DIV_ONEHOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `refcount_div_onehot` writer - "]
pub type REFCOUNT_DIV_ONEHOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROC_MON_SPEC, u8, u8, 4, O>;
#[doc = "Field `ring_freq` reader - "]
pub type RING_FREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ring_freq_rdy` reader - "]
pub type RING_FREQ_RDY_R = crate::BitReader<bool>;
#[doc = "Field `reserved_29_31` reader - "]
pub type RESERVED_29_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_proc_mon(&self) -> PU_PROC_MON_R {
        PU_PROC_MON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_en_rvt(&self) -> OSC_EN_RVT_R {
        OSC_EN_RVT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn osc_en_lvt(&self) -> OSC_EN_LVT_R {
        OSC_EN_LVT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn osc_sel(&self) -> OSC_SEL_R {
        OSC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rstn_ringcount(&self) -> RSTN_RINGCOUNT_R {
        RSTN_RINGCOUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rstn_refcount(&self) -> RSTN_REFCOUNT_R {
        RSTN_REFCOUNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_6_7(&self) -> RESERVED_6_7_R {
        RESERVED_6_7_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn refcount_div_onehot(&self) -> REFCOUNT_DIV_ONEHOT_R {
        REFCOUNT_DIV_ONEHOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    pub fn ring_freq(&self) -> RING_FREQ_R {
        RING_FREQ_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ring_freq_rdy(&self) -> RING_FREQ_RDY_R {
        RING_FREQ_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn reserved_29_31(&self) -> RESERVED_29_31_R {
        RESERVED_29_31_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_proc_mon(&mut self) -> PU_PROC_MON_W<0> {
        PU_PROC_MON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn osc_en_rvt(&mut self) -> OSC_EN_RVT_W<1> {
        OSC_EN_RVT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn osc_en_lvt(&mut self) -> OSC_EN_LVT_W<2> {
        OSC_EN_LVT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn osc_sel(&mut self) -> OSC_SEL_W<3> {
        OSC_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_ringcount(&mut self) -> RSTN_RINGCOUNT_W<4> {
        RSTN_RINGCOUNT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_refcount(&mut self) -> RSTN_REFCOUNT_W<5> {
        RSTN_REFCOUNT_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn refcount_div_onehot(&mut self) -> REFCOUNT_DIV_ONEHOT_W<8> {
        REFCOUNT_DIV_ONEHOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "proc_mon\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [proc_mon](index.html) module"]
pub struct PROC_MON_SPEC;
impl crate::RegisterSpec for PROC_MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc_mon::R](R) reader structure"]
impl crate::Readable for PROC_MON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc_mon::W](W) writer structure"]
impl crate::Writable for PROC_MON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets proc_mon to value 0x0400"]
impl crate::Resettable for PROC_MON_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
