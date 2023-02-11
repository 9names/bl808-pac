#[doc = "Register `tzc_mm_ctrl_0` reader"]
pub struct R(crate::R<TZC_MM_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_MM_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_MM_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_MM_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_mm_ctrl_0` writer"]
pub struct W(crate::W<TZC_MM_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_MM_CTRL_0_SPEC>;
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
impl From<crate::W<TZC_MM_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_MM_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_mm_pwron_rst_tzsid_en` reader - "]
pub type TZC_MM_PWRON_RST_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_cpu0_reset_tzsid_en` reader - "]
pub type TZC_MM_CPU0_RESET_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_sys_reset_tzsid_en` reader - "]
pub type TZC_MM_SYS_RESET_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_cpu0_tzsid_en` reader - "]
pub type TZC_MM_CPU0_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_8_9` reader - "]
pub type RESERVED_8_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_sram_tzsid_en` reader - "]
pub type TZC_MM_SRAM_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_swrst_tzsid_en` reader - "]
pub type TZC_MM_SWRST_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_14_19` reader - "]
pub type RESERVED_14_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_mm_clk_tzsid_en` reader - "]
pub type TZC_MM_CLK_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_22_31` reader - "]
pub type RESERVED_22_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_mm_pwron_rst_tzsid_en(&self) -> TZC_MM_PWRON_RST_TZSID_EN_R {
        TZC_MM_PWRON_RST_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_reset_tzsid_en(&self) -> TZC_MM_CPU0_RESET_TZSID_EN_R {
        TZC_MM_CPU0_RESET_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_mm_sys_reset_tzsid_en(&self) -> TZC_MM_SYS_RESET_TZSID_EN_R {
        TZC_MM_SYS_RESET_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_mm_cpu0_tzsid_en(&self) -> TZC_MM_CPU0_TZSID_EN_R {
        TZC_MM_CPU0_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reserved_8_9(&self) -> RESERVED_8_9_R {
        RESERVED_8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_mm_sram_tzsid_en(&self) -> TZC_MM_SRAM_TZSID_EN_R {
        TZC_MM_SRAM_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_mm_swrst_tzsid_en(&self) -> TZC_MM_SWRST_TZSID_EN_R {
        TZC_MM_SWRST_TZSID_EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn reserved_14_19(&self) -> RESERVED_14_19_R {
        RESERVED_14_19_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_mm_clk_tzsid_en(&self) -> TZC_MM_CLK_TZSID_EN_R {
        TZC_MM_CLK_TZSID_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn reserved_22_31(&self) -> RESERVED_22_31_R {
        RESERVED_22_31_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_mm_ctrl_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_mm_ctrl_0](index.html) module"]
pub struct TZC_MM_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_MM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_mm_ctrl_0::R](R) reader structure"]
impl crate::Readable for TZC_MM_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_mm_ctrl_0::W](W) writer structure"]
impl crate::Writable for TZC_MM_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_ctrl_0 to value 0x0030_3cff"]
impl crate::Resettable for TZC_MM_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_3cff;
}
