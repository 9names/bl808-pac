#[doc = "Register `hbn_bor_cfg` reader"]
pub struct R(crate::R<HBN_BOR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_BOR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_BOR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_BOR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbn_bor_cfg` writer"]
pub struct W(crate::W<HBN_BOR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_BOR_CFG_SPEC>;
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
impl From<crate::W<HBN_BOR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_BOR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bod_sel` reader - "]
pub type BOD_SEL_R = crate::BitReader<bool>;
#[doc = "Field `bod_sel` writer - "]
pub type BOD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_BOR_CFG_SPEC, bool, O>;
#[doc = "Field `bod_vth` reader - "]
pub type BOD_VTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bod_vth` writer - "]
pub type BOD_VTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_BOR_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `pu_bod` reader - "]
pub type PU_BOD_R = crate::BitReader<bool>;
#[doc = "Field `pu_bod` writer - "]
pub type PU_BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_BOR_CFG_SPEC, bool, O>;
#[doc = "Field `r_bod_out` reader - "]
pub type R_BOD_OUT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bod_sel(&self) -> BOD_SEL_R {
        BOD_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn bod_vth(&self) -> BOD_VTH_R {
        BOD_VTH_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_bod(&self) -> PU_BOD_R {
        PU_BOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_bod_out(&self) -> R_BOD_OUT_R {
        R_BOD_OUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved_6_31(&self) -> RESERVED_6_31_R {
        RESERVED_6_31_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bod_sel(&mut self) -> BOD_SEL_W<0> {
        BOD_SEL_W::new(self)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn bod_vth(&mut self) -> BOD_VTH_W<1> {
        BOD_VTH_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bod(&mut self) -> PU_BOD_W<4> {
        PU_BOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_BOR_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_bor_cfg](index.html) module"]
pub struct HBN_BOR_CFG_SPEC;
impl crate::RegisterSpec for HBN_BOR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_bor_cfg::R](R) reader structure"]
impl crate::Readable for HBN_BOR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_bor_cfg::W](W) writer structure"]
impl crate::Writable for HBN_BOR_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbn_bor_cfg to value 0x0a"]
impl crate::Resettable for HBN_BOR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
