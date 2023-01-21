#[doc = "Register `uart_cfg2` reader"]
pub struct R(crate::R<UART_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_cfg2` writer"]
pub struct W(crate::W<UART_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CFG2_SPEC>;
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
impl From<crate::W<UART_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_sig_8_sel` reader - "]
pub type UART_SIG_8_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_8_sel` writer - "]
pub type UART_SIG_8_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_9_sel` reader - "]
pub type UART_SIG_9_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_9_sel` writer - "]
pub type UART_SIG_9_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_10_sel` reader - "]
pub type UART_SIG_10_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_10_sel` writer - "]
pub type UART_SIG_10_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `uart_sig_11_sel` reader - "]
pub type UART_SIG_11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart_sig_11_sel` writer - "]
pub type UART_SIG_11_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CFG2_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_16_31` reader - "]
pub type RESERVED_16_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_8_sel(&self) -> UART_SIG_8_SEL_R {
        UART_SIG_8_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_9_sel(&self) -> UART_SIG_9_SEL_R {
        UART_SIG_9_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_10_sel(&self) -> UART_SIG_10_SEL_R {
        UART_SIG_10_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_11_sel(&self) -> UART_SIG_11_SEL_R {
        UART_SIG_11_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_8_sel(&mut self) -> UART_SIG_8_SEL_W<0> {
        UART_SIG_8_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_9_sel(&mut self) -> UART_SIG_9_SEL_W<4> {
        UART_SIG_9_SEL_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_10_sel(&mut self) -> UART_SIG_10_SEL_W<8> {
        UART_SIG_10_SEL_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn uart_sig_11_sel(&mut self) -> UART_SIG_11_SEL_W<12> {
        UART_SIG_11_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cfg2](index.html) module"]
pub struct UART_CFG2_SPEC;
impl crate::RegisterSpec for UART_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_cfg2::R](R) reader structure"]
impl crate::Readable for UART_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_cfg2::W](W) writer structure"]
impl crate::Writable for UART_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_cfg2 to value 0xba98"]
impl crate::Resettable for UART_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xba98;
}
