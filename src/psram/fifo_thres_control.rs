#[doc = "Register `fifo_thres_control` reader"]
pub struct R(crate::R<FIFO_THRES_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_THRES_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_THRES_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_THRES_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fifo_thres_control` writer"]
pub struct W(crate::W<FIFO_THRES_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_THRES_CONTROL_SPEC>;
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
impl From<crate::W<FIFO_THRES_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_THRES_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mask_w_fifo_cnt` reader - "]
pub type REG_MASK_W_FIFO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mask_w_fifo_cnt` writer - "]
pub type REG_MASK_W_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIFO_THRES_CONTROL_SPEC, u16, u16, 16, O>;
#[doc = "Field `reg_mask_r_fifo_rem` reader - "]
pub type REG_MASK_R_FIFO_REM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mask_r_fifo_rem` writer - "]
pub type REG_MASK_R_FIFO_REM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FIFO_THRES_CONTROL_SPEC, u16, u16, 16, O>;
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
#[doc = "fifo_thres_control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_thres_control](index.html) module"]
pub struct FIFO_THRES_CONTROL_SPEC;
impl crate::RegisterSpec for FIFO_THRES_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_thres_control::R](R) reader structure"]
impl crate::Readable for FIFO_THRES_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_thres_control::W](W) writer structure"]
impl crate::Writable for FIFO_THRES_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fifo_thres_control to value 0"]
impl crate::Resettable for FIFO_THRES_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
