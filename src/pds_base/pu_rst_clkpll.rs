#[doc = "Register `pu_rst_clkpll` reader"]
pub struct R(crate::R<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PU_RST_CLKPLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PU_RST_CLKPLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PU_RST_CLKPLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pu_rst_clkpll` writer"]
pub struct W(crate::W<PU_RST_CLKPLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PU_RST_CLKPLL_SPEC>;
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
impl From<crate::W<PU_RST_CLKPLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PU_RST_CLKPLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0_8` reader - "]
pub type RESERVED_0_8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` reader - "]
pub type CR_PDS_PU_CLKPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` writer - "]
pub type CR_PDS_PU_CLKPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `cr_pds_pu_clkpll` reader - "]
pub type CR_PDS_PU_CLKPLL_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pu_clkpll` writer - "]
pub type CR_PDS_PU_CLKPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PU_RST_CLKPLL_SPEC, bool, O>;
#[doc = "Field `reserved_11_31` reader - "]
pub type RESERVED_11_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn reserved_0_8(&self) -> RESERVED_0_8_R {
        RESERVED_0_8_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll_sfreg(&self) -> CR_PDS_PU_CLKPLL_SFREG_R {
        CR_PDS_PU_CLKPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll(&self) -> CR_PDS_PU_CLKPLL_R {
        CR_PDS_PU_CLKPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn reserved_11_31(&self) -> RESERVED_11_31_R {
        RESERVED_11_31_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pu_clkpll_sfreg(&mut self) -> CR_PDS_PU_CLKPLL_SFREG_W<9> {
        CR_PDS_PU_CLKPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pu_clkpll(&mut self) -> CR_PDS_PU_CLKPLL_W<10> {
        CR_PDS_PU_CLKPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pu_rst_clkpll\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pu_rst_clkpll](index.html) module"]
pub struct PU_RST_CLKPLL_SPEC;
impl crate::RegisterSpec for PU_RST_CLKPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pu_rst_clkpll::R](R) reader structure"]
impl crate::Readable for PU_RST_CLKPLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pu_rst_clkpll::W](W) writer structure"]
impl crate::Writable for PU_RST_CLKPLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pu_rst_clkpll to value 0"]
impl crate::Resettable for PU_RST_CLKPLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
