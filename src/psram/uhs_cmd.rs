#[doc = "Register `uhs_cmd` reader"]
pub struct R(crate::R<UHS_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_cmd` writer"]
pub struct W(crate::W<UHS_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_CMD_SPEC>;
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
impl From<crate::W<UHS_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_glbr_pulse` writer - "]
pub type REG_GLBR_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_CMD_SPEC, bool, O>;
#[doc = "Field `reg_srfi_pulse` writer - "]
pub type REG_SRFI_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_CMD_SPEC, bool, O>;
#[doc = "Field `reg_srfo_pulse` writer - "]
pub type REG_SRFO_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_CMD_SPEC, bool, O>;
#[doc = "Field `reg_regw_pulse` writer - "]
pub type REG_REGW_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_CMD_SPEC, bool, O>;
#[doc = "Field `reg_regr_pulse` writer - "]
pub type REG_REGR_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_CMD_SPEC, bool, O>;
#[doc = "Field `reserved_5_7` reader - "]
pub type RESERVED_5_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_glbr_done` reader - "]
pub type STS_GLBR_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_srfi_done` reader - "]
pub type STS_SRFI_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_srfo_done` reader - "]
pub type STS_SRFO_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_regw_done` reader - "]
pub type STS_REGW_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_regr_done` reader - "]
pub type STS_REGR_DONE_R = crate::BitReader<bool>;
#[doc = "Field `sts_init_done` reader - "]
pub type STS_INIT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_14_23` reader - "]
pub type RESERVED_14_23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_config_read` reader - "]
pub type STS_CONFIG_READ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved_5_7(&self) -> RESERVED_5_7_R {
        RESERVED_5_7_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_glbr_done(&self) -> STS_GLBR_DONE_R {
        STS_GLBR_DONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_srfi_done(&self) -> STS_SRFI_DONE_R {
        STS_SRFI_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sts_srfo_done(&self) -> STS_SRFO_DONE_R {
        STS_SRFO_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sts_regw_done(&self) -> STS_REGW_DONE_R {
        STS_REGW_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sts_regr_done(&self) -> STS_REGR_DONE_R {
        STS_REGR_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sts_init_done(&self) -> STS_INIT_DONE_R {
        STS_INIT_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:23"]
    #[inline(always)]
    pub fn reserved_14_23(&self) -> RESERVED_14_23_R {
        RESERVED_14_23_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_config_read(&self) -> STS_CONFIG_READ_R {
        STS_CONFIG_READ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_glbr_pulse(&mut self) -> REG_GLBR_PULSE_W<0> {
        REG_GLBR_PULSE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_srfi_pulse(&mut self) -> REG_SRFI_PULSE_W<1> {
        REG_SRFI_PULSE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_srfo_pulse(&mut self) -> REG_SRFO_PULSE_W<2> {
        REG_SRFO_PULSE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_regw_pulse(&mut self) -> REG_REGW_PULSE_W<3> {
        REG_REGW_PULSE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_regr_pulse(&mut self) -> REG_REGR_PULSE_W<4> {
        REG_REGR_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_cmd\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_cmd](index.html) module"]
pub struct UHS_CMD_SPEC;
impl crate::RegisterSpec for UHS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_cmd::R](R) reader structure"]
impl crate::Readable for UHS_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_cmd::W](W) writer structure"]
impl crate::Writable for UHS_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_cmd to value 0"]
impl crate::Resettable for UHS_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
