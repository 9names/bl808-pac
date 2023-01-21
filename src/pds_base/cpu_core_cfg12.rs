#[doc = "Register `cpu_core_cfg12` reader"]
pub struct R(crate::R<CPU_CORE_CFG12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CORE_CFG12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CORE_CFG12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CORE_CFG12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cpu_core_cfg12` writer"]
pub struct W(crate::W<CPU_CORE_CFG12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CORE_CFG12_SPEC>;
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
impl From<crate::W<CPU_CORE_CFG12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CORE_CFG12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `e902_iahbl_base` reader - "]
pub type E902_IAHBL_BASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `e902_iahbl_base` writer - "]
pub type E902_IAHBL_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CORE_CFG12_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `e902_iahbl_mask` reader - "]
pub type E902_IAHBL_MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `e902_iahbl_mask` writer - "]
pub type E902_IAHBL_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CORE_CFG12_SPEC, u16, u16, 12, O>;
#[doc = "Field `reserved_28_31` reader - "]
pub type RESERVED_28_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn e902_iahbl_base(&self) -> E902_IAHBL_BASE_R {
        E902_IAHBL_BASE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn e902_iahbl_mask(&self) -> E902_IAHBL_MASK_R {
        E902_IAHBL_MASK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reserved_28_31(&self) -> RESERVED_28_31_R {
        RESERVED_28_31_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn e902_iahbl_base(&mut self) -> E902_IAHBL_BASE_W<0> {
        E902_IAHBL_BASE_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn e902_iahbl_mask(&mut self) -> E902_IAHBL_MASK_W<16> {
        E902_IAHBL_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cpu_core_cfg12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_core_cfg12](index.html) module"]
pub struct CPU_CORE_CFG12_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_core_cfg12::R](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_core_cfg12::W](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg12 to value 0"]
impl crate::Resettable for CPU_CORE_CFG12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
