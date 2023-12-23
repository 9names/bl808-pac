#[doc = "Register `psram_intf_delay_ctrl2` reader"]
pub struct R(crate::R<PSRAM_INTF_DELAY_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_INTF_DELAY_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_INTF_DELAY_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_INTF_DELAY_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_intf_delay_ctrl2` writer"]
pub struct W(crate::W<PSRAM_INTF_DELAY_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_INTF_DELAY_CTRL2_SPEC>;
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
impl From<crate::W<PSRAM_INTF_DELAY_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_INTF_DELAY_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_delay_sel_o_adq5` reader - "]
pub type REG_DELAY_SEL_O_ADQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq5` writer - "]
pub type REG_DELAY_SEL_O_ADQ5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_adq4` reader - "]
pub type REG_DELAY_SEL_O_ADQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq4` writer - "]
pub type REG_DELAY_SEL_O_ADQ4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_adq3` reader - "]
pub type REG_DELAY_SEL_O_ADQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq3` writer - "]
pub type REG_DELAY_SEL_O_ADQ3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_adq2` reader - "]
pub type REG_DELAY_SEL_O_ADQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq2` writer - "]
pub type REG_DELAY_SEL_O_ADQ2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq5(&self) -> REG_DELAY_SEL_O_ADQ5_R {
        REG_DELAY_SEL_O_ADQ5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq4(&self) -> REG_DELAY_SEL_O_ADQ4_R {
        REG_DELAY_SEL_O_ADQ4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq3(&self) -> REG_DELAY_SEL_O_ADQ3_R {
        REG_DELAY_SEL_O_ADQ3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq2(&self) -> REG_DELAY_SEL_O_ADQ2_R {
        REG_DELAY_SEL_O_ADQ2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq5(&mut self) -> REG_DELAY_SEL_O_ADQ5_W<0> {
        REG_DELAY_SEL_O_ADQ5_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq4(&mut self) -> REG_DELAY_SEL_O_ADQ4_W<8> {
        REG_DELAY_SEL_O_ADQ4_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq3(&mut self) -> REG_DELAY_SEL_O_ADQ3_W<16> {
        REG_DELAY_SEL_O_ADQ3_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq2(&mut self) -> REG_DELAY_SEL_O_ADQ2_W<24> {
        REG_DELAY_SEL_O_ADQ2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_intf_delay_ctrl2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_intf_delay_ctrl2](index.html) module"]
pub struct PSRAM_INTF_DELAY_CTRL2_SPEC;
impl crate::RegisterSpec for PSRAM_INTF_DELAY_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_intf_delay_ctrl2::R](R) reader structure"]
impl crate::Readable for PSRAM_INTF_DELAY_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_intf_delay_ctrl2::W](W) writer structure"]
impl crate::Writable for PSRAM_INTF_DELAY_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_intf_delay_ctrl2 to value 0"]
impl crate::Resettable for PSRAM_INTF_DELAY_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
