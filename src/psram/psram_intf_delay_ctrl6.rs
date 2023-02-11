#[doc = "Register `psram_intf_delay_ctrl6` reader"]
pub struct R(crate::R<PSRAM_INTF_DELAY_CTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSRAM_INTF_DELAY_CTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSRAM_INTF_DELAY_CTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSRAM_INTF_DELAY_CTRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `psram_intf_delay_ctrl6` writer"]
pub struct W(crate::W<PSRAM_INTF_DELAY_CTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSRAM_INTF_DELAY_CTRL6_SPEC>;
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
impl From<crate::W<PSRAM_INTF_DELAY_CTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSRAM_INTF_DELAY_CTRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_delay_sel_o_adq9` reader - "]
pub type REG_DELAY_SEL_O_ADQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq9` writer - "]
pub type REG_DELAY_SEL_O_ADQ9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL6_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_adq8` reader - "]
pub type REG_DELAY_SEL_O_ADQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq8` writer - "]
pub type REG_DELAY_SEL_O_ADQ8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL6_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_adq_oen1` reader - "]
pub type REG_DELAY_SEL_O_ADQ_OEN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_adq_oen1` writer - "]
pub type REG_DELAY_SEL_O_ADQ_OEN1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL6_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_delay_sel_o_dqs1` reader - "]
pub type REG_DELAY_SEL_O_DQS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_delay_sel_o_dqs1` writer - "]
pub type REG_DELAY_SEL_O_DQS1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSRAM_INTF_DELAY_CTRL6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq9(&self) -> REG_DELAY_SEL_O_ADQ9_R {
        REG_DELAY_SEL_O_ADQ9_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq8(&self) -> REG_DELAY_SEL_O_ADQ8_R {
        REG_DELAY_SEL_O_ADQ8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq_oen1(&self) -> REG_DELAY_SEL_O_ADQ_OEN1_R {
        REG_DELAY_SEL_O_ADQ_OEN1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs1(&self) -> REG_DELAY_SEL_O_DQS1_R {
        REG_DELAY_SEL_O_DQS1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq9(&mut self) -> REG_DELAY_SEL_O_ADQ9_W<0> {
        REG_DELAY_SEL_O_ADQ9_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq8(&mut self) -> REG_DELAY_SEL_O_ADQ8_W<8> {
        REG_DELAY_SEL_O_ADQ8_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_adq_oen1(&mut self) -> REG_DELAY_SEL_O_ADQ_OEN1_W<16> {
        REG_DELAY_SEL_O_ADQ_OEN1_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_delay_sel_o_dqs1(&mut self) -> REG_DELAY_SEL_O_DQS1_W<24> {
        REG_DELAY_SEL_O_DQS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "psram_intf_delay_ctrl6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psram_intf_delay_ctrl6](index.html) module"]
pub struct PSRAM_INTF_DELAY_CTRL6_SPEC;
impl crate::RegisterSpec for PSRAM_INTF_DELAY_CTRL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psram_intf_delay_ctrl6::R](R) reader structure"]
impl crate::Readable for PSRAM_INTF_DELAY_CTRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psram_intf_delay_ctrl6::W](W) writer structure"]
impl crate::Writable for PSRAM_INTF_DELAY_CTRL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_intf_delay_ctrl6 to value 0"]
impl crate::Resettable for PSRAM_INTF_DELAY_CTRL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
