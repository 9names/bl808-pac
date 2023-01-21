#[doc = "Register `tzc_glb_ctrl_0` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_0` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_0_SPEC>;
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
impl From<crate::W<TZC_GLB_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GLB_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` reader - "]
pub type TZC_GLB_PWRON_RST_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_pwron_rst_tzsid_en` writer - "]
pub type TZC_GLB_PWRON_RST_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` reader - "]
pub type TZC_GLB_CPU_RESET_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_cpu_reset_tzsid_en` writer - "]
pub type TZC_GLB_CPU_RESET_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` reader - "]
pub type TZC_GLB_SYS_RESET_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_sys_reset_tzsid_en` writer - "]
pub type TZC_GLB_SYS_RESET_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` reader - "]
pub type TZC_GLB_CPU2_RESET_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_cpu2_reset_tzsid_en` writer - "]
pub type TZC_GLB_CPU2_RESET_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_misc_tzsid_en` reader - "]
pub type TZC_GLB_MISC_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_misc_tzsid_en` writer - "]
pub type TZC_GLB_MISC_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_sram_tzsid_en` reader - "]
pub type TZC_GLB_SRAM_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_sram_tzsid_en` writer - "]
pub type TZC_GLB_SRAM_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_swrst_tzsid_en` reader - "]
pub type TZC_GLB_SWRST_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_swrst_tzsid_en` writer - "]
pub type TZC_GLB_SWRST_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_bmx_tzsid_en` reader - "]
pub type TZC_GLB_BMX_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_bmx_tzsid_en` writer - "]
pub type TZC_GLB_BMX_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_dbg_tzsid_en` reader - "]
pub type TZC_GLB_DBG_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_dbg_tzsid_en` writer - "]
pub type TZC_GLB_DBG_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_mbist_tzsid_en` reader - "]
pub type TZC_GLB_MBIST_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_mbist_tzsid_en` writer - "]
pub type TZC_GLB_MBIST_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_clk_tzsid_en` reader - "]
pub type TZC_GLB_CLK_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_clk_tzsid_en` writer - "]
pub type TZC_GLB_CLK_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_int_tzsid_en` reader - "]
pub type TZC_GLB_INT_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_int_tzsid_en` writer - "]
pub type TZC_GLB_INT_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_glb_pwr_tzsid_en` reader - "]
pub type TZC_GLB_PWR_TZSID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_glb_pwr_tzsid_en` writer - "]
pub type TZC_GLB_PWR_TZSID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_GLB_CTRL_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_26_31` reader - "]
pub type RESERVED_26_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_glb_pwron_rst_tzsid_en(&self) -> TZC_GLB_PWRON_RST_TZSID_EN_R {
        TZC_GLB_PWRON_RST_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_glb_cpu_reset_tzsid_en(&self) -> TZC_GLB_CPU_RESET_TZSID_EN_R {
        TZC_GLB_CPU_RESET_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_glb_sys_reset_tzsid_en(&self) -> TZC_GLB_SYS_RESET_TZSID_EN_R {
        TZC_GLB_SYS_RESET_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&self) -> TZC_GLB_CPU2_RESET_TZSID_EN_R {
        TZC_GLB_CPU2_RESET_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_glb_misc_tzsid_en(&self) -> TZC_GLB_MISC_TZSID_EN_R {
        TZC_GLB_MISC_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_glb_sram_tzsid_en(&self) -> TZC_GLB_SRAM_TZSID_EN_R {
        TZC_GLB_SRAM_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_glb_swrst_tzsid_en(&self) -> TZC_GLB_SWRST_TZSID_EN_R {
        TZC_GLB_SWRST_TZSID_EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_glb_bmx_tzsid_en(&self) -> TZC_GLB_BMX_TZSID_EN_R {
        TZC_GLB_BMX_TZSID_EN_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_glb_dbg_tzsid_en(&self) -> TZC_GLB_DBG_TZSID_EN_R {
        TZC_GLB_DBG_TZSID_EN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_glb_mbist_tzsid_en(&self) -> TZC_GLB_MBIST_TZSID_EN_R {
        TZC_GLB_MBIST_TZSID_EN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_glb_clk_tzsid_en(&self) -> TZC_GLB_CLK_TZSID_EN_R {
        TZC_GLB_CLK_TZSID_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_glb_int_tzsid_en(&self) -> TZC_GLB_INT_TZSID_EN_R {
        TZC_GLB_INT_TZSID_EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_glb_pwr_tzsid_en(&self) -> TZC_GLB_PWR_TZSID_EN_R {
        TZC_GLB_PWR_TZSID_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn reserved_26_31(&self) -> RESERVED_26_31_R {
        RESERVED_26_31_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwron_rst_tzsid_en(&mut self) -> TZC_GLB_PWRON_RST_TZSID_EN_W<0> {
        TZC_GLB_PWRON_RST_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu_reset_tzsid_en(&mut self) -> TZC_GLB_CPU_RESET_TZSID_EN_W<2> {
        TZC_GLB_CPU_RESET_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sys_reset_tzsid_en(&mut self) -> TZC_GLB_SYS_RESET_TZSID_EN_W<4> {
        TZC_GLB_SYS_RESET_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_cpu2_reset_tzsid_en(&mut self) -> TZC_GLB_CPU2_RESET_TZSID_EN_W<6> {
        TZC_GLB_CPU2_RESET_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_misc_tzsid_en(&mut self) -> TZC_GLB_MISC_TZSID_EN_W<8> {
        TZC_GLB_MISC_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_sram_tzsid_en(&mut self) -> TZC_GLB_SRAM_TZSID_EN_W<10> {
        TZC_GLB_SRAM_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_swrst_tzsid_en(&mut self) -> TZC_GLB_SWRST_TZSID_EN_W<12> {
        TZC_GLB_SWRST_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_bmx_tzsid_en(&mut self) -> TZC_GLB_BMX_TZSID_EN_W<14> {
        TZC_GLB_BMX_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_dbg_tzsid_en(&mut self) -> TZC_GLB_DBG_TZSID_EN_W<16> {
        TZC_GLB_DBG_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_mbist_tzsid_en(&mut self) -> TZC_GLB_MBIST_TZSID_EN_W<18> {
        TZC_GLB_MBIST_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_clk_tzsid_en(&mut self) -> TZC_GLB_CLK_TZSID_EN_W<20> {
        TZC_GLB_CLK_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_int_tzsid_en(&mut self) -> TZC_GLB_INT_TZSID_EN_W<22> {
        TZC_GLB_INT_TZSID_EN_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_pwr_tzsid_en(&mut self) -> TZC_GLB_PWR_TZSID_EN_W<24> {
        TZC_GLB_PWR_TZSID_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_0](index.html) module"]
pub struct TZC_GLB_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_0::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_0::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_0 to value 0x03ff_ffff"]
impl crate::Resettable for TZC_GLB_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_ffff;
}
