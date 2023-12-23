#[doc = "Register `io_dly_3` reader"]
pub struct R(crate::R<IO_DLY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_DLY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_DLY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_DLY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `io_dly_3` writer"]
pub struct W(crate::W<IO_DLY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_DLY_3_SPEC>;
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
impl From<crate::W<IO_DLY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_DLY_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_2_oe_dly_sel` reader - "]
pub type IO_2_OE_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `io_2_oe_dly_sel` writer - "]
pub type IO_2_OE_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_DLY_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_2_7` reader - "]
pub type RESERVED_2_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `io_2_di_dly_sel` reader - "]
pub type IO_2_DI_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `io_2_di_dly_sel` writer - "]
pub type IO_2_DI_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_DLY_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_10_15` reader - "]
pub type RESERVED_10_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `io_2_do_dly_sel` reader - "]
pub type IO_2_DO_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `io_2_do_dly_sel` writer - "]
pub type IO_2_DO_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IO_DLY_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_18_31` reader - "]
pub type RESERVED_18_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn io_2_oe_dly_sel(&self) -> IO_2_OE_DLY_SEL_R {
        IO_2_OE_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn reserved_2_7(&self) -> RESERVED_2_7_R {
        RESERVED_2_7_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn io_2_di_dly_sel(&self) -> IO_2_DI_DLY_SEL_R {
        IO_2_DI_DLY_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn reserved_10_15(&self) -> RESERVED_10_15_R {
        RESERVED_10_15_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn io_2_do_dly_sel(&self) -> IO_2_DO_DLY_SEL_R {
        IO_2_DO_DLY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn reserved_18_31(&self) -> RESERVED_18_31_R {
        RESERVED_18_31_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn io_2_oe_dly_sel(&mut self) -> IO_2_OE_DLY_SEL_W<0> {
        IO_2_OE_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn io_2_di_dly_sel(&mut self) -> IO_2_DI_DLY_SEL_W<8> {
        IO_2_DI_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn io_2_do_dly_sel(&mut self) -> IO_2_DO_DLY_SEL_W<16> {
        IO_2_DO_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "if_io_dly_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_dly_3](index.html) module"]
pub struct IO_DLY_3_SPEC;
impl crate::RegisterSpec for IO_DLY_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_dly_3::R](R) reader structure"]
impl crate::Readable for IO_DLY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_dly_3::W](W) writer structure"]
impl crate::Writable for IO_DLY_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets io_dly_3 to value 0"]
impl crate::Resettable for IO_DLY_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
