#[doc = "Register `uhs_auto_fresh_2` reader"]
pub struct R(crate::R<UHS_AUTO_FRESH_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_AUTO_FRESH_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_AUTO_FRESH_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_AUTO_FRESH_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_auto_fresh_2` writer"]
pub struct W(crate::W<UHS_AUTO_FRESH_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_AUTO_FRESH_2_SPEC>;
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
impl From<crate::W<UHS_AUTO_FRESH_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_AUTO_FRESH_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_refi_cycle` reader - "]
pub type REG_REFI_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_refi_cycle` writer - "]
pub type REG_REFI_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_AUTO_FRESH_2_SPEC, u16, u16, 16, O>;
#[doc = "Field `reg_win_ref_cnt` reader - "]
pub type REG_WIN_REF_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_win_ref_cnt` writer - "]
pub type REG_WIN_REF_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_AUTO_FRESH_2_SPEC, u16, u16, 13, O>;
#[doc = "Field `reserved_29_31` reader - "]
pub type RESERVED_29_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_refi_cycle(&self) -> REG_REFI_CYCLE_R {
        REG_REFI_CYCLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn reg_win_ref_cnt(&self) -> REG_WIN_REF_CNT_R {
        REG_WIN_REF_CNT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn reserved_29_31(&self) -> RESERVED_29_31_R {
        RESERVED_29_31_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_refi_cycle(&mut self) -> REG_REFI_CYCLE_W<0> {
        REG_REFI_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_win_ref_cnt(&mut self) -> REG_WIN_REF_CNT_W<16> {
        REG_WIN_REF_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_auto_fresh_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_auto_fresh_2](index.html) module"]
pub struct UHS_AUTO_FRESH_2_SPEC;
impl crate::RegisterSpec for UHS_AUTO_FRESH_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_auto_fresh_2::R](R) reader structure"]
impl crate::Readable for UHS_AUTO_FRESH_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_auto_fresh_2::W](W) writer structure"]
impl crate::Writable for UHS_AUTO_FRESH_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_auto_fresh_2 to value 0x1000_0027"]
impl crate::Resettable for UHS_AUTO_FRESH_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0027;
}
