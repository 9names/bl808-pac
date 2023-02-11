#[doc = "Register `sd_fifo_param` reader"]
pub struct R(crate::R<SD_FIFO_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_FIFO_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_FIFO_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_FIFO_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_fifo_param` writer"]
pub struct W(crate::W<SD_FIFO_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_FIFO_PARAM_SPEC>;
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
impl From<crate::W<SD_FIFO_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_FIFO_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc` reader - "]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc` writer - "]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_FIFO_PARAM_SPEC, u8, u8, 2, O>;
#[doc = "Field `wtc` reader - "]
pub type WTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wtc` writer - "]
pub type WTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_FIFO_PARAM_SPEC, u8, u8, 2, O>;
#[doc = "Field `fifo_clk` reader - "]
pub type FIFO_CLK_R = crate::BitReader<bool>;
#[doc = "Field `fifo_clk` writer - "]
pub type FIFO_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `fifo_cs` reader - "]
pub type FIFO_CS_R = crate::BitReader<bool>;
#[doc = "Field `fifo_cs` writer - "]
pub type FIFO_CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `pdwn` reader - "]
pub type PDWN_R = crate::BitReader<bool>;
#[doc = "Field `pdwn` writer - "]
pub type PDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `use_dat3` reader - "]
pub type USE_DAT3_R = crate::BitReader<bool>;
#[doc = "Field `use_dat3` writer - "]
pub type USE_DAT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `clk_gate_ctl` reader - "]
pub type CLK_GATE_CTL_R = crate::BitReader<bool>;
#[doc = "Field `clk_gate_ctl` writer - "]
pub type CLK_GATE_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `clk_gate_on` reader - "]
pub type CLK_GATE_ON_R = crate::BitReader<bool>;
#[doc = "Field `clk_gate_on` writer - "]
pub type CLK_GATE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `reserved_10` reader - "]
pub type RESERVED_10_R = crate::BitReader<bool>;
#[doc = "Field `ovrrd_clk_oen` reader - "]
pub type OVRRD_CLK_OEN_R = crate::BitReader<bool>;
#[doc = "Field `ovrrd_clk_oen` writer - "]
pub type OVRRD_CLK_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `force_clk_on` reader - "]
pub type FORCE_CLK_ON_R = crate::BitReader<bool>;
#[doc = "Field `force_clk_on` writer - "]
pub type FORCE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `pdfvssm` reader - "]
pub type PDFVSSM_R = crate::BitReader<bool>;
#[doc = "Field `pdfvssm` writer - "]
pub type PDFVSSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `pdlvmc` reader - "]
pub type PDLVMC_R = crate::BitReader<bool>;
#[doc = "Field `pdlvmc` writer - "]
pub type PDLVMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `pre_gate_clk_cnt` reader - "]
pub type PRE_GATE_CLK_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pre_gate_clk_cnt` writer - "]
pub type PRE_GATE_CLK_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_FIFO_PARAM_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_31_20` reader - "]
pub type RESERVED_31_20_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn wtc(&self) -> WTC_R {
        WTC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_clk(&self) -> FIFO_CLK_R {
        FIFO_CLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fifo_cs(&self) -> FIFO_CS_R {
        FIFO_CS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn use_dat3(&self) -> USE_DAT3_R {
        USE_DAT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_gate_ctl(&self) -> CLK_GATE_CTL_R {
        CLK_GATE_CTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_gate_on(&self) -> CLK_GATE_ON_R {
        CLK_GATE_ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reserved_10(&self) -> RESERVED_10_R {
        RESERVED_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovrrd_clk_oen(&self) -> OVRRD_CLK_OEN_R {
        OVRRD_CLK_OEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn force_clk_on(&self) -> FORCE_CLK_ON_R {
        FORCE_CLK_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pdfvssm(&self) -> PDFVSSM_R {
        PDFVSSM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pdlvmc(&self) -> PDLVMC_R {
        PDLVMC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pre_gate_clk_cnt(&self) -> PRE_GATE_CLK_CNT_R {
        PRE_GATE_CLK_CNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn reserved_31_20(&self) -> RESERVED_31_20_R {
        RESERVED_31_20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn wtc(&mut self) -> WTC_W<2> {
        WTC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_clk(&mut self) -> FIFO_CLK_W<4> {
        FIFO_CLK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_cs(&mut self) -> FIFO_CS_W<5> {
        FIFO_CS_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<6> {
        PDWN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn use_dat3(&mut self) -> USE_DAT3_W<7> {
        USE_DAT3_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gate_ctl(&mut self) -> CLK_GATE_CTL_W<8> {
        CLK_GATE_CTL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gate_on(&mut self) -> CLK_GATE_ON_W<9> {
        CLK_GATE_ON_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrd_clk_oen(&mut self) -> OVRRD_CLK_OEN_W<11> {
        OVRRD_CLK_OEN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn force_clk_on(&mut self) -> FORCE_CLK_ON_W<12> {
        FORCE_CLK_ON_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pdfvssm(&mut self) -> PDFVSSM_W<13> {
        PDFVSSM_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pdlvmc(&mut self) -> PDLVMC_W<14> {
        PDLVMC_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pre_gate_clk_cnt(&mut self) -> PRE_GATE_CLK_CNT_W<16> {
        PRE_GATE_CLK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_fifo_param](index.html) module"]
pub struct SD_FIFO_PARAM_SPEC;
impl crate::RegisterSpec for SD_FIFO_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_fifo_param::R](R) reader structure"]
impl crate::Readable for SD_FIFO_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_fifo_param::W](W) writer structure"]
impl crate::Writable for SD_FIFO_PARAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_fifo_param to value 0x0007_0005"]
impl crate::Resettable for SD_FIFO_PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0005;
}
