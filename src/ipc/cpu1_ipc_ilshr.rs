#[doc = "Register `cpu1_ipc_ilshr` reader"]
pub struct R(crate::R<CPU1_IPC_ILSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU1_IPC_ILSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU1_IPC_ILSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU1_IPC_ILSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu1_ipc_ilshr` writer"]
pub struct W(crate::W<CPU1_IPC_ILSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU1_IPC_ILSHR_SPEC>;
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
impl From<crate::W<CPU1_IPC_ILSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU1_IPC_ILSHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu1_ipc_ilshr` reader - "]
pub type CPU1_IPC_ILSHR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cpu1_ipc_ilshr` writer - "]
pub type CPU1_IPC_ILSHR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU1_IPC_ILSHR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu1_ipc_ilshr(&self) -> CPU1_IPC_ILSHR_R {
        CPU1_IPC_ILSHR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_ipc_ilshr(&mut self) -> CPU1_IPC_ILSHR_W<0> {
        CPU1_IPC_ILSHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Line Sel High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu1_ipc_ilshr](index.html) module"]
pub struct CPU1_IPC_ILSHR_SPEC;
impl crate::RegisterSpec for CPU1_IPC_ILSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu1_ipc_ilshr::R](R) reader structure"]
impl crate::Readable for CPU1_IPC_ILSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu1_ipc_ilshr::W](W) writer structure"]
impl crate::Writable for CPU1_IPC_ILSHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu1_ipc_ilshr to value 0"]
impl crate::Resettable for CPU1_IPC_ILSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
