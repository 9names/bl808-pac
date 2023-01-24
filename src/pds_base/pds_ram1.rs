#[doc = "Register `pds_ram1` reader"]
pub struct R(crate::R<PDS_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_RAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ram1` writer"]
pub struct W(crate::W<PDS_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_RAM1_SPEC>;
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
impl From<crate::W<PDS_RAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_ocram_slp` reader - "]
pub type CR_OCRAM_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ocram_slp` writer - "]
pub type CR_OCRAM_SLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_ocram_ret` reader - "]
pub type CR_OCRAM_RET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ocram_ret` writer - "]
pub type CR_OCRAM_RET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_pds_ram_clk_cnt` reader - "]
pub type CR_PDS_RAM_CLK_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_clk_cnt` writer - "]
pub type CR_PDS_RAM_CLK_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_clk2_cnt` reader - "]
pub type CR_PDS_RAM_CLK2_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_clk2_cnt` writer - "]
pub type CR_PDS_RAM_CLK2_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 6, O>;
#[doc = "Field `reserved_22_23` reader - "]
pub type RESERVED_22_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` reader - "]
pub type CR_PDS_CTRL_NP_RAM_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ctrl_np_ram_clk` writer - "]
pub type CR_PDS_CTRL_NP_RAM_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_RAM1_SPEC, bool, O>;
#[doc = "Field `cr_pds_ctrl_mm_ram_clk` reader - "]
pub type CR_PDS_CTRL_MM_RAM_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ctrl_mm_ram_clk` writer - "]
pub type CR_PDS_CTRL_MM_RAM_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_RAM1_SPEC, bool, O>;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` reader - "]
pub type CR_PDS_CTRL_WB_RAM_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ctrl_wb_ram_clk` writer - "]
pub type CR_PDS_CTRL_WB_RAM_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_RAM1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_ocram_slp(&self) -> CR_OCRAM_SLP_R {
        CR_OCRAM_SLP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_ocram_ret(&self) -> CR_OCRAM_RET_R {
        CR_OCRAM_RET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_ram_clk_cnt(&self) -> CR_PDS_RAM_CLK_CNT_R {
        CR_PDS_RAM_CLK_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_ram_clk2_cnt(&self) -> CR_PDS_RAM_CLK2_CNT_R {
        CR_PDS_RAM_CLK2_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn reserved_22_23(&self) -> RESERVED_22_23_R {
        RESERVED_22_23_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_ctrl_np_ram_clk(&self) -> CR_PDS_CTRL_NP_RAM_CLK_R {
        CR_PDS_CTRL_NP_RAM_CLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_ctrl_mm_ram_clk(&self) -> CR_PDS_CTRL_MM_RAM_CLK_R {
        CR_PDS_CTRL_MM_RAM_CLK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_ctrl_wb_ram_clk(&self) -> CR_PDS_CTRL_WB_RAM_CLK_R {
        CR_PDS_CTRL_WB_RAM_CLK_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_slp(&mut self) -> CR_OCRAM_SLP_W<0> {
        CR_OCRAM_SLP_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_ret(&mut self) -> CR_OCRAM_RET_W<4> {
        CR_OCRAM_RET_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ram_clk_cnt(&mut self) -> CR_PDS_RAM_CLK_CNT_W<8> {
        CR_PDS_RAM_CLK_CNT_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ram_clk2_cnt(&mut self) -> CR_PDS_RAM_CLK2_CNT_W<16> {
        CR_PDS_RAM_CLK2_CNT_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_np_ram_clk(&mut self) -> CR_PDS_CTRL_NP_RAM_CLK_W<24> {
        CR_PDS_CTRL_NP_RAM_CLK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_mm_ram_clk(&mut self) -> CR_PDS_CTRL_MM_RAM_CLK_W<25> {
        CR_PDS_CTRL_MM_RAM_CLK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_ctrl_wb_ram_clk(&mut self) -> CR_PDS_CTRL_WB_RAM_CLK_W<26> {
        CR_PDS_CTRL_WB_RAM_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_ram1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram1](index.html) module"]
pub struct PDS_RAM1_SPEC;
impl crate::RegisterSpec for PDS_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ram1::R](R) reader structure"]
impl crate::Readable for PDS_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ram1::W](W) writer structure"]
impl crate::Writable for PDS_RAM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_ram1 to value 0x0018_0800"]
impl crate::Resettable for PDS_RAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0018_0800;
}
