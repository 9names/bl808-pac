#[doc = "Register `cpu1_ipc_isr` reader"]
pub struct R(crate::R<CPU1_IPC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU1_IPC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU1_IPC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU1_IPC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu1_ipc_isr` writer"]
pub struct W(crate::W<CPU1_IPC_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU1_IPC_ISR_SPEC>;
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
impl From<crate::W<CPU1_IPC_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU1_IPC_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu1_ipc_isr` reader - "]
pub type CPU1_IPC_ISR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reserved_31_16` reader - "]
pub type RESERVED_31_16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu1_ipc_isr(&self) -> CPU1_IPC_ISR_R {
        CPU1_IPC_ISR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_31_16(&self) -> RESERVED_31_16_R {
        RESERVED_31_16_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Interrupt status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu1_ipc_isr](index.html) module"]
pub struct CPU1_IPC_ISR_SPEC;
impl crate::RegisterSpec for CPU1_IPC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu1_ipc_isr::R](R) reader structure"]
impl crate::Readable for CPU1_IPC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu1_ipc_isr::W](W) writer structure"]
impl crate::Writable for CPU1_IPC_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu1_ipc_isr to value 0"]
impl crate::Resettable for CPU1_IPC_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
