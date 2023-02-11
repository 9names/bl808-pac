#[doc = "Register `pds_ram2` reader"]
pub struct R(crate::R<PDS_RAM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_RAM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_RAM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_RAM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ram2` writer"]
pub struct W(crate::W<PDS_RAM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_RAM2_SPEC>;
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
impl From<crate::W<PDS_RAM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_RAM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_wram_slp` reader - "]
pub type CR_WRAM_SLP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_wram_slp` writer - "]
pub type CR_WRAM_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM2_SPEC, u16, u16, 10, O>;
#[doc = "Field `cr_wram_ret` reader - "]
pub type CR_WRAM_RET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_wram_ret` writer - "]
pub type CR_WRAM_RET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM2_SPEC, u16, u16, 10, O>;
#[doc = "Field `reserved_20_31` reader - "]
pub type RESERVED_20_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cr_wram_slp(&self) -> CR_WRAM_SLP_R {
        CR_WRAM_SLP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn cr_wram_ret(&self) -> CR_WRAM_RET_R {
        CR_WRAM_RET_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn reserved_20_31(&self) -> RESERVED_20_31_R {
        RESERVED_20_31_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_slp(&mut self) -> CR_WRAM_SLP_W<0> {
        CR_WRAM_SLP_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_ret(&mut self) -> CR_WRAM_RET_W<10> {
        CR_WRAM_RET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_RAM2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram2](index.html) module"]
pub struct PDS_RAM2_SPEC;
impl crate::RegisterSpec for PDS_RAM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ram2::R](R) reader structure"]
impl crate::Readable for PDS_RAM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ram2::W](W) writer structure"]
impl crate::Writable for PDS_RAM2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_ram2 to value 0"]
impl crate::Resettable for PDS_RAM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
