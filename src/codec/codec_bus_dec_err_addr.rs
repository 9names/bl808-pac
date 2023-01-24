#[doc = "Register `codec_bus_dec_err_addr` reader"]
pub struct R(crate::R<CODEC_BUS_DEC_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEC_BUS_DEC_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEC_BUS_DEC_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEC_BUS_DEC_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `codec_bus_dec_err_addr` writer"]
pub struct W(crate::W<CODEC_BUS_DEC_ERR_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEC_BUS_DEC_ERR_ADDR_SPEC>;
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
impl From<crate::W<CODEC_BUS_DEC_ERR_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEC_BUS_DEC_ERR_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `codec_hs_dec_err_addr` reader - "]
pub type CODEC_HS_DEC_ERR_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn codec_hs_dec_err_addr(&self) -> CODEC_HS_DEC_ERR_ADDR_R {
        CODEC_HS_DEC_ERR_ADDR_R::new(self.bits)
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
#[doc = "codec_bus_dec_err_addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codec_bus_dec_err_addr](index.html) module"]
pub struct CODEC_BUS_DEC_ERR_ADDR_SPEC;
impl crate::RegisterSpec for CODEC_BUS_DEC_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codec_bus_dec_err_addr::R](R) reader structure"]
impl crate::Readable for CODEC_BUS_DEC_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codec_bus_dec_err_addr::W](W) writer structure"]
impl crate::Writable for CODEC_BUS_DEC_ERR_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets codec_bus_dec_err_addr to value 0"]
impl crate::Resettable for CODEC_BUS_DEC_ERR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
