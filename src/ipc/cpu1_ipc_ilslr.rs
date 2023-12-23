#[doc = "Register `cpu1_ipc_ilslr` reader"]
pub struct R(crate::R<CPU1_IPC_ILSLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU1_IPC_ILSLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU1_IPC_ILSLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU1_IPC_ILSLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu1_ipc_ilslr` writer"]
pub struct W(crate::W<CPU1_IPC_ILSLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU1_IPC_ILSLR_SPEC>;
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
impl From<crate::W<CPU1_IPC_ILSLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU1_IPC_ILSLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu1_ipc_ilslr` reader - "]
pub type CPU1_IPC_ILSLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cpu1_ipc_ilslr` writer - "]
pub type CPU1_IPC_ILSLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU1_IPC_ILSLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu1_ipc_ilslr(&self) -> CPU1_IPC_ILSLR_R {
        CPU1_IPC_ILSLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_ipc_ilslr(&mut self) -> CPU1_IPC_ILSLR_W<0> {
        CPU1_IPC_ILSLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Line Sel Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu1_ipc_ilslr](index.html) module"]
pub struct CPU1_IPC_ILSLR_SPEC;
impl crate::RegisterSpec for CPU1_IPC_ILSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu1_ipc_ilslr::R](R) reader structure"]
impl crate::Readable for CPU1_IPC_ILSLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu1_ipc_ilslr::W](W) writer structure"]
impl crate::Writable for CPU1_IPC_ILSLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu1_ipc_ilslr to value 0"]
impl crate::Resettable for CPU1_IPC_ILSLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
