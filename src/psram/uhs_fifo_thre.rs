#[doc = "Register `uhs_fifo_thre` reader"]
pub struct R(crate::R<UHS_FIFO_THRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_FIFO_THRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_FIFO_THRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_FIFO_THRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_fifo_thre` writer"]
pub struct W(crate::W<UHS_FIFO_THRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_FIFO_THRE_SPEC>;
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
impl From<crate::W<UHS_FIFO_THRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_FIFO_THRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mask_w_fifo_cnt` reader - "]
pub type REG_MASK_W_FIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mask_w_fifo_cnt` writer - "]
pub type REG_MASK_W_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_FIFO_THRE_SPEC, u16, u16, 16, O>;
#[doc = "Field `reg_mask_r_fifo_rem` reader - "]
pub type REG_MASK_R_FIFO_REM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mask_r_fifo_rem` writer - "]
pub type REG_MASK_R_FIFO_REM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_FIFO_THRE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_mask_w_fifo_cnt(&self) -> REG_MASK_W_FIFO_CNT_R {
        REG_MASK_W_FIFO_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_mask_r_fifo_rem(&self) -> REG_MASK_R_FIFO_REM_R {
        REG_MASK_R_FIFO_REM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mask_w_fifo_cnt(&mut self) -> REG_MASK_W_FIFO_CNT_W<0> {
        REG_MASK_W_FIFO_CNT_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mask_r_fifo_rem(&mut self) -> REG_MASK_R_FIFO_REM_W<16> {
        REG_MASK_R_FIFO_REM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_fifo_thre\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_fifo_thre](index.html) module"]
pub struct UHS_FIFO_THRE_SPEC;
impl crate::RegisterSpec for UHS_FIFO_THRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_fifo_thre::R](R) reader structure"]
impl crate::Readable for UHS_FIFO_THRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_fifo_thre::W](W) writer structure"]
impl crate::Writable for UHS_FIFO_THRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_fifo_thre to value 0"]
impl crate::Resettable for UHS_FIFO_THRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
