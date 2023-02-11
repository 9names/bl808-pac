#[doc = "Register `uhs_rsvd_reg` reader"]
pub struct R(crate::R<UHS_RSVD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_RSVD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_RSVD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_RSVD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_rsvd_reg` writer"]
pub struct W(crate::W<UHS_RSVD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_RSVD_REG_SPEC>;
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
impl From<crate::W<UHS_RSVD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_RSVD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mr0_7` reader - "]
pub type REG_MR0_7_R = crate::BitReader<bool>;
#[doc = "Field `reg_mr0_7` writer - "]
pub type REG_MR0_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_RSVD_REG_SPEC, bool, O>;
#[doc = "Field `reg_mr2_2_0` reader - "]
pub type REG_MR2_2_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mr2_2_0` writer - "]
pub type REG_MR2_2_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_RSVD_REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_mr2_7_6` reader - "]
pub type REG_MR2_7_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mr2_7_6` writer - "]
pub type REG_MR2_7_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_RSVD_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_31` reader - "]
pub type RESERVED_6_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mr0_7(&self) -> REG_MR0_7_R {
        REG_MR0_7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reg_mr2_2_0(&self) -> REG_MR2_2_0_R {
        REG_MR2_2_0_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_mr2_7_6(&self) -> REG_MR2_7_6_R {
        REG_MR2_7_6_R::new(((self.bits >> 4) & 3) as u8)
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
    pub fn reg_mr0_7(&mut self) -> REG_MR0_7_W<0> {
        REG_MR0_7_W::new(self)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mr2_2_0(&mut self) -> REG_MR2_2_0_W<1> {
        REG_MR2_2_0_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mr2_7_6(&mut self) -> REG_MR2_7_6_W<4> {
        REG_MR2_7_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_rsvd_reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_rsvd_reg](index.html) module"]
pub struct UHS_RSVD_REG_SPEC;
impl crate::RegisterSpec for UHS_RSVD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_rsvd_reg::R](R) reader structure"]
impl crate::Readable for UHS_RSVD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_rsvd_reg::W](W) writer structure"]
impl crate::Writable for UHS_RSVD_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_rsvd_reg to value 0"]
impl crate::Resettable for UHS_RSVD_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
