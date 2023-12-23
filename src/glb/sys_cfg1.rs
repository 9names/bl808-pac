#[doc = "Register `sys_cfg1` reader"]
pub struct R(crate::R<SYS_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sys_cfg1` writer"]
pub struct W(crate::W<SYS_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CFG1_SPEC>;
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
impl From<crate::W<SYS_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bclk_div_act_pulse` writer - "]
pub type REG_BCLK_DIV_ACT_PULSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bclk_div_bypass` reader - "]
pub type REG_BCLK_DIV_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `reg_bclk_div_bypass` writer - "]
pub type REG_BCLK_DIV_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CFG1_SPEC, bool, O>;
#[doc = "Field `sts_bclk_prot_done` reader - "]
pub type STS_BCLK_PROT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `reg_bclk_sw_done_cnt` reader - "]
pub type REG_BCLK_SW_DONE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_bclk_sw_done_cnt` writer - "]
pub type REG_BCLK_SW_DONE_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_8_15` reader - "]
pub type RESERVED_8_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pico_clk_div_act_pulse` writer - "]
pub type REG_PICO_CLK_DIV_ACT_PULSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_pico_clk_div_bypass` reader - "]
pub type REG_PICO_CLK_DIV_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `reg_pico_clk_div_bypass` writer - "]
pub type REG_PICO_CLK_DIV_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_CFG1_SPEC, bool, O>;
#[doc = "Field `sts_pico_clk_prot_done` reader - "]
pub type STS_PICO_CLK_PROT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `reg_pico_clk_sw_done_cnt` reader - "]
pub type REG_PICO_CLK_SW_DONE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pico_clk_sw_done_cnt` writer - "]
pub type REG_PICO_CLK_SW_DONE_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `fclk_sw_state` reader - "]
pub type FCLK_SW_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_27_31` reader - "]
pub type RESERVED_27_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bclk_div_bypass(&self) -> REG_BCLK_DIV_BYPASS_R {
        REG_BCLK_DIV_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_bclk_prot_done(&self) -> STS_BCLK_PROT_DONE_R {
        STS_BCLK_PROT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_bclk_sw_done_cnt(&self) -> REG_BCLK_SW_DONE_CNT_R {
        REG_BCLK_SW_DONE_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reserved_8_15(&self) -> RESERVED_8_15_R {
        RESERVED_8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_pico_clk_div_bypass(&self) -> REG_PICO_CLK_DIV_BYPASS_R {
        REG_PICO_CLK_DIV_BYPASS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sts_pico_clk_prot_done(&self) -> STS_PICO_CLK_PROT_DONE_R {
        STS_PICO_CLK_PROT_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_pico_clk_sw_done_cnt(&self) -> REG_PICO_CLK_SW_DONE_CNT_R {
        REG_PICO_CLK_SW_DONE_CNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FCLK_SW_STATE_R {
        FCLK_SW_STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn reserved_27_31(&self) -> RESERVED_27_31_R {
        RESERVED_27_31_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_div_act_pulse(&mut self) -> REG_BCLK_DIV_ACT_PULSE_W<0> {
        REG_BCLK_DIV_ACT_PULSE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_div_bypass(&mut self) -> REG_BCLK_DIV_BYPASS_W<1> {
        REG_BCLK_DIV_BYPASS_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bclk_sw_done_cnt(&mut self) -> REG_BCLK_SW_DONE_CNT_W<4> {
        REG_BCLK_SW_DONE_CNT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pico_clk_div_act_pulse(&mut self) -> REG_PICO_CLK_DIV_ACT_PULSE_W<16> {
        REG_PICO_CLK_DIV_ACT_PULSE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pico_clk_div_bypass(&mut self) -> REG_PICO_CLK_DIV_BYPASS_W<17> {
        REG_PICO_CLK_DIV_BYPASS_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pico_clk_sw_done_cnt(&mut self) -> REG_PICO_CLK_SW_DONE_CNT_W<20> {
        REG_PICO_CLK_SW_DONE_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sys_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_cfg1](index.html) module"]
pub struct SYS_CFG1_SPEC;
impl crate::RegisterSpec for SYS_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_cfg1::R](R) reader structure"]
impl crate::Readable for SYS_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_cfg1::W](W) writer structure"]
impl crate::Writable for SYS_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_cfg1 to value 0x0054_0054"]
impl crate::Resettable for SYS_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0054_0054;
}
