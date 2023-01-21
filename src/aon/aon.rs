#[doc = "Register `aon` reader"]
pub struct R(crate::R<AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon` writer"]
pub struct W(crate::W<AON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_SPEC>;
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
impl From<crate::W<AON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aon_resv` reader - "]
pub type AON_RESV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `aon_resv` writer - "]
pub type AON_RESV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AON_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_8_11` reader - "]
pub type RESERVED_8_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pu_aon_dc_tbuf` reader - "]
pub type PU_AON_DC_TBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_aon_dc_tbuf` writer - "]
pub type PU_AON_DC_TBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_SPEC, bool, O>;
#[doc = "Field `reserved_13_19` reader - "]
pub type RESERVED_13_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ldo11_rt_pulldown` reader - "]
pub type LDO11_RT_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ldo11_rt_pulldown` writer - "]
pub type LDO11_RT_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_SPEC, bool, O>;
#[doc = "Field `ldo11_rt_pulldown_sel` reader - "]
pub type LDO11_RT_PULLDOWN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ldo11_rt_pulldown_sel` writer - "]
pub type LDO11_RT_PULLDOWN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_SPEC, bool, O>;
#[doc = "Field `sw_pu_ldo11_rt` reader - "]
pub type SW_PU_LDO11_RT_R = crate::BitReader<bool>;
#[doc = "Field `sw_pu_ldo11_rt` writer - "]
pub type SW_PU_LDO11_RT_W<'a, const O: u8> = crate::BitWriter<'a, u32, AON_SPEC, bool, O>;
#[doc = "Field `reserved_23_31` reader - "]
pub type RESERVED_23_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&self) -> AON_RESV_R {
        AON_RESV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reserved_8_11(&self) -> RESERVED_8_11_R {
        RESERVED_8_11_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&self) -> PU_AON_DC_TBUF_R {
        PU_AON_DC_TBUF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:19"]
    #[inline(always)]
    pub fn reserved_13_19(&self) -> RESERVED_13_19_R {
        RESERVED_13_19_R::new(((self.bits >> 13) & 0x7f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&self) -> LDO11_RT_PULLDOWN_R {
        LDO11_RT_PULLDOWN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&self) -> LDO11_RT_PULLDOWN_SEL_R {
        LDO11_RT_PULLDOWN_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&self) -> SW_PU_LDO11_RT_R {
        SW_PU_LDO11_RT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn reserved_23_31(&self) -> RESERVED_23_31_R {
        RESERVED_23_31_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn aon_resv(&mut self) -> AON_RESV_W<0> {
        AON_RESV_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aon_dc_tbuf(&mut self) -> PU_AON_DC_TBUF_W<12> {
        PU_AON_DC_TBUF_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11_rt_pulldown(&mut self) -> LDO11_RT_PULLDOWN_W<20> {
        LDO11_RT_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11_rt_pulldown_sel(&mut self) -> LDO11_RT_PULLDOWN_SEL_W<21> {
        LDO11_RT_PULLDOWN_SEL_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pu_ldo11_rt(&mut self) -> SW_PU_LDO11_RT_W<22> {
        SW_PU_LDO11_RT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon](index.html) module"]
pub struct AON_SPEC;
impl crate::RegisterSpec for AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon::R](R) reader structure"]
impl crate::Readable for AON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon::W](W) writer structure"]
impl crate::Writable for AON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon to value 0x0040_000f"]
impl crate::Resettable for AON_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_000f;
}
