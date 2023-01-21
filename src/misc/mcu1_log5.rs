#[doc = "Register `mcu1_log5` reader"]
pub struct R(crate::R<MCU1_LOG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU1_LOG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU1_LOG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU1_LOG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu1_log5` writer"]
pub struct W(crate::W<MCU1_LOG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU1_LOG5_SPEC>;
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
impl From<crate::W<MCU1_LOG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU1_LOG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_23` reader - "]
pub type RESERVED_0_23_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sts_mcu1_lockup` reader - "]
pub type STS_MCU1_LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `sts_mcu1_halted` reader - "]
pub type STS_MCU1_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mcu1_ndm_rstn_req` reader - "]
pub type MCU1_NDM_RSTN_REQ_R = crate::BitReader<bool>;
#[doc = "Field `mcu1_hart_rstn_req` reader - "]
pub type MCU1_HART_RSTN_REQ_R = crate::BitReader<bool>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn reserved_0_23(&self) -> RESERVED_0_23_R {
        RESERVED_0_23_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_mcu1_lockup(&self) -> STS_MCU1_LOCKUP_R {
        STS_MCU1_LOCKUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sts_mcu1_halted(&self) -> STS_MCU1_HALTED_R {
        STS_MCU1_HALTED_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_req(&self) -> MCU1_NDM_RSTN_REQ_R {
        MCU1_NDM_RSTN_REQ_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mcu1_hart_rstn_req(&self) -> MCU1_HART_RSTN_REQ_R {
        MCU1_HART_RSTN_REQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu1_log5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu1_log5](index.html) module"]
pub struct MCU1_LOG5_SPEC;
impl crate::RegisterSpec for MCU1_LOG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu1_log5::R](R) reader structure"]
impl crate::Readable for MCU1_LOG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu1_log5::W](W) writer structure"]
impl crate::Writable for MCU1_LOG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log5 to value 0"]
impl crate::Resettable for MCU1_LOG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
