#[doc = "Register `tzc_pds` reader"]
pub struct R(crate::R<TZC_PDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_pds` writer"]
pub struct W(crate::W<TZC_PDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_PDS_SPEC>;
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
impl From<crate::W<TZC_PDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_PDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_e902_cfg_wr_lock` reader - "]
pub type CR_E902_CFG_WR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `cr_e902_cfg_wr_lock` writer - "]
pub type CR_E902_CFG_WR_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_PDS_SPEC, bool, O>;
#[doc = "Field `cr_e906_cfg_wr_lock` reader - "]
pub type CR_E906_CFG_WR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `cr_e906_cfg_wr_lock` writer - "]
pub type CR_E906_CFG_WR_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZC_PDS_SPEC, bool, O>;
#[doc = "Field `reserved_2_31` reader - "]
pub type RESERVED_2_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_e902_cfg_wr_lock(&self) -> CR_E902_CFG_WR_LOCK_R {
        CR_E902_CFG_WR_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_e906_cfg_wr_lock(&self) -> CR_E906_CFG_WR_LOCK_R {
        CR_E906_CFG_WR_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn reserved_2_31(&self) -> RESERVED_2_31_R {
        RESERVED_2_31_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_e902_cfg_wr_lock(&mut self) -> CR_E902_CFG_WR_LOCK_W<0> {
        CR_E902_CFG_WR_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_e906_cfg_wr_lock(&mut self) -> CR_E906_CFG_WR_LOCK_W<1> {
        CR_E906_CFG_WR_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_pds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pds](index.html) module"]
pub struct TZC_PDS_SPEC;
impl crate::RegisterSpec for TZC_PDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_pds::R](R) reader structure"]
impl crate::Readable for TZC_PDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_pds::W](W) writer structure"]
impl crate::Writable for TZC_PDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_pds to value 0"]
impl crate::Resettable for TZC_PDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
