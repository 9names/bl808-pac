#[doc = "Register `codec_bus_dec_err` reader"]
pub struct R(crate::R<CODEC_BUS_DEC_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEC_BUS_DEC_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEC_BUS_DEC_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEC_BUS_DEC_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `codec_bus_dec_err` writer"]
pub struct W(crate::W<CODEC_BUS_DEC_ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEC_BUS_DEC_ERR_SPEC>;
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
impl From<crate::W<CODEC_BUS_DEC_ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEC_BUS_DEC_ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dec_err_clr` reader - "]
pub type REG_DEC_ERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_dec_err_clr` writer - "]
pub type REG_DEC_ERR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_BUS_DEC_ERR_SPEC, bool, O>;
#[doc = "Field `reg_dec_err_latch_last` reader - "]
pub type REG_DEC_ERR_LATCH_LAST_R = crate::BitReader<bool>;
#[doc = "Field `reg_dec_err_latch_last` writer - "]
pub type REG_DEC_ERR_LATCH_LAST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_BUS_DEC_ERR_SPEC, bool, O>;
#[doc = "Field `reserved_2_7` reader - "]
pub type RESERVED_2_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `codec_hs_dec_err_lat` reader - "]
pub type CODEC_HS_DEC_ERR_LAT_R = crate::BitReader<bool>;
#[doc = "Field `codec_hs_dec_err_write` reader - "]
pub type CODEC_HS_DEC_ERR_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_10_11` reader - "]
pub type RESERVED_10_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `codec_hs_dec_err_src` reader - "]
pub type CODEC_HS_DEC_ERR_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `codec_hs_dec_err_id` reader - "]
pub type CODEC_HS_DEC_ERR_ID_R = crate::BitReader<bool>;
#[doc = "Field `reserved_17_31` reader - "]
pub type RESERVED_17_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_err_clr(&self) -> REG_DEC_ERR_CLR_R {
        REG_DEC_ERR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_dec_err_latch_last(&self) -> REG_DEC_ERR_LATCH_LAST_R {
        REG_DEC_ERR_LATCH_LAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn reserved_2_7(&self) -> RESERVED_2_7_R {
        RESERVED_2_7_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn codec_hs_dec_err_lat(&self) -> CODEC_HS_DEC_ERR_LAT_R {
        CODEC_HS_DEC_ERR_LAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn codec_hs_dec_err_write(&self) -> CODEC_HS_DEC_ERR_WRITE_R {
        CODEC_HS_DEC_ERR_WRITE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reserved_10_11(&self) -> RESERVED_10_11_R {
        RESERVED_10_11_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn codec_hs_dec_err_src(&self) -> CODEC_HS_DEC_ERR_SRC_R {
        CODEC_HS_DEC_ERR_SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn codec_hs_dec_err_id(&self) -> CODEC_HS_DEC_ERR_ID_R {
        CODEC_HS_DEC_ERR_ID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserved_17_31(&self) -> RESERVED_17_31_R {
        RESERVED_17_31_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dec_err_clr(&mut self) -> REG_DEC_ERR_CLR_W<0> {
        REG_DEC_ERR_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dec_err_latch_last(&mut self) -> REG_DEC_ERR_LATCH_LAST_W<1> {
        REG_DEC_ERR_LATCH_LAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "codec_bus_dec_err\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codec_bus_dec_err](index.html) module"]
pub struct CODEC_BUS_DEC_ERR_SPEC;
impl crate::RegisterSpec for CODEC_BUS_DEC_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codec_bus_dec_err::R](R) reader structure"]
impl crate::Readable for CODEC_BUS_DEC_ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codec_bus_dec_err::W](W) writer structure"]
impl crate::Writable for CODEC_BUS_DEC_ERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets codec_bus_dec_err to value 0"]
impl crate::Resettable for CODEC_BUS_DEC_ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
