#[doc = "Register `sram_cfg3` reader"]
pub struct R(crate::R<SRAM_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_cfg3` writer"]
pub struct W(crate::W<SRAM_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CFG3_SPEC>;
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
impl From<crate::W<SRAM_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `em_sel` reader - "]
pub type EM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `em_sel` writer - "]
pub type EM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAM_CFG3_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_8_27` reader - "]
pub type RESERVED_8_27_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_vram_sel` reader - "]
pub type REG_VRAM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_vram_sel` writer - "]
pub type REG_VRAM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn em_sel(&self) -> EM_SEL_R {
        EM_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27"]
    #[inline(always)]
    pub fn reserved_8_27(&self) -> RESERVED_8_27_R {
        RESERVED_8_27_R::new((self.bits >> 8) & 0x000f_ffff)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_vram_sel(&self) -> REG_VRAM_SEL_R {
        REG_VRAM_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn em_sel(&mut self) -> EM_SEL_W<0> {
        EM_SEL_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vram_sel(&mut self) -> REG_VRAM_SEL_W<28> {
        REG_VRAM_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sram_cfg3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cfg3](index.html) module"]
pub struct SRAM_CFG3_SPEC;
impl crate::RegisterSpec for SRAM_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cfg3::R](R) reader structure"]
impl crate::Readable for SRAM_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cfg3::W](W) writer structure"]
impl crate::Writable for SRAM_CFG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_cfg3 to value 0x03"]
impl crate::Resettable for SRAM_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
