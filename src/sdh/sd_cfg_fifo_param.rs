#[doc = "Register `sd_cfg_fifo_param` reader"]
pub struct R(crate::R<SD_CFG_FIFO_PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CFG_FIFO_PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CFG_FIFO_PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CFG_FIFO_PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_cfg_fifo_param` writer"]
pub struct W(crate::W<SD_CFG_FIFO_PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CFG_FIFO_PARAM_SPEC>;
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
impl From<crate::W<SD_CFG_FIFO_PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CFG_FIFO_PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0` reader - "]
pub type RESERVED_0_R = crate::BitReader<bool>;
#[doc = "Field `reserved_2_1` reader - "]
pub type RESERVED_2_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `boot_ack` reader - "]
pub type BOOT_ACK_R = crate::BitReader<bool>;
#[doc = "Field `boot_ack` writer - "]
pub type BOOT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CFG_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `squ_empty_chk` reader - "]
pub type SQU_EMPTY_CHK_R = crate::BitReader<bool>;
#[doc = "Field `squ_empty_chk` writer - "]
pub type SQU_EMPTY_CHK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SD_CFG_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `squ_full_chk` reader - "]
pub type SQU_FULL_CHK_R = crate::BitReader<bool>;
#[doc = "Field `squ_full_chk` writer - "]
pub type SQU_FULL_CHK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SD_CFG_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `gen_pad_clk_on` reader - "]
pub type GEN_PAD_CLK_ON_R = crate::BitReader<bool>;
#[doc = "Field `gen_pad_clk_on` writer - "]
pub type GEN_PAD_CLK_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SD_CFG_FIFO_PARAM_SPEC, bool, O>;
#[doc = "Field `reserved_23_7` reader - "]
pub type RESERVED_23_7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `gen_pad_clk_cnt` reader - "]
pub type GEN_PAD_CLK_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gen_pad_clk_cnt` writer - "]
pub type GEN_PAD_CLK_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_CFG_FIFO_PARAM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> RESERVED_0_R {
        RESERVED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn reserved_2_1(&self) -> RESERVED_2_1_R {
        RESERVED_2_1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn boot_ack(&self) -> BOOT_ACK_R {
        BOOT_ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn squ_empty_chk(&self) -> SQU_EMPTY_CHK_R {
        SQU_EMPTY_CHK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn squ_full_chk(&self) -> SQU_FULL_CHK_R {
        SQU_FULL_CHK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gen_pad_clk_on(&self) -> GEN_PAD_CLK_ON_R {
        GEN_PAD_CLK_ON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:23"]
    #[inline(always)]
    pub fn reserved_23_7(&self) -> RESERVED_23_7_R {
        RESERVED_23_7_R::new((self.bits >> 7) & 0x0001_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gen_pad_clk_cnt(&self) -> GEN_PAD_CLK_CNT_R {
        GEN_PAD_CLK_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W<3> {
        BOOT_ACK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn squ_empty_chk(&mut self) -> SQU_EMPTY_CHK_W<4> {
        SQU_EMPTY_CHK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn squ_full_chk(&mut self) -> SQU_FULL_CHK_W<5> {
        SQU_FULL_CHK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pad_clk_on(&mut self) -> GEN_PAD_CLK_ON_W<6> {
        GEN_PAD_CLK_ON_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pad_clk_cnt(&mut self) -> GEN_PAD_CLK_CNT_W<24> {
        GEN_PAD_CLK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Extra Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_cfg_fifo_param](index.html) module"]
pub struct SD_CFG_FIFO_PARAM_SPEC;
impl crate::RegisterSpec for SD_CFG_FIFO_PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_cfg_fifo_param::R](R) reader structure"]
impl crate::Readable for SD_CFG_FIFO_PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_cfg_fifo_param::W](W) writer structure"]
impl crate::Writable for SD_CFG_FIFO_PARAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_cfg_fifo_param to value 0x4a00_0000"]
impl crate::Resettable for SD_CFG_FIFO_PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x4a00_0000;
}
