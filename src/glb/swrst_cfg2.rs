#[doc = "Register `swrst_cfg2` reader"]
pub struct R(crate::R<SWRST_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg2` writer"]
pub struct W(crate::W<SWRST_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG2_SPEC>;
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
impl From<crate::W<SWRST_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRST_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_ctrl_pwron_rst` reader - "]
pub type REG_CTRL_PWRON_RST_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_pwron_rst` writer - "]
pub type REG_CTRL_PWRON_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_cpu_reset` reader - "]
pub type REG_CTRL_CPU_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_cpu_reset` writer - "]
pub type REG_CTRL_CPU_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_sys_reset` reader - "]
pub type REG_CTRL_SYS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_sys_reset` writer - "]
pub type REG_CTRL_SYS_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_pico_reset` reader - "]
pub type REG_CTRL_PICO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_pico_reset` writer - "]
pub type REG_CTRL_PICO_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_cpu2_reset` reader - "]
pub type REG_CTRL_CPU2_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_cpu2_reset` writer - "]
pub type REG_CTRL_CPU2_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_ctrl_chip_reset` reader - "]
pub type REG_CTRL_CHIP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `reg_ctrl_chip_reset` writer - "]
pub type REG_CTRL_CHIP_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_wl_wdt_reset_mm_en` reader - "]
pub type REG_WL_WDT_RESET_MM_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_wl_wdt_reset_mm_en` writer - "]
pub type REG_WL_WDT_RESET_MM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reg_mmwdt2wl_rst_msk` reader - "]
pub type REG_MMWDT2WL_RST_MSK_R = crate::BitReader<bool>;
#[doc = "Field `reg_mmwdt2wl_rst_msk` writer - "]
pub type REG_MMWDT2WL_RST_MSK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_8_23` reader - "]
pub type RESERVED_8_23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pka_clk_sel` reader - "]
pub type PKA_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `pka_clk_sel` writer - "]
pub type PKA_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRST_CFG2_SPEC, bool, O>;
#[doc = "Field `reserved_25_27` reader - "]
pub type RESERVED_25_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ctrl_reset_dummy` reader - "]
pub type REG_CTRL_RESET_DUMMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_ctrl_reset_dummy` writer - "]
pub type REG_CTRL_RESET_DUMMY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWRST_CFG2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> REG_CTRL_PWRON_RST_R {
        REG_CTRL_PWRON_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> REG_CTRL_CPU_RESET_R {
        REG_CTRL_CPU_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> REG_CTRL_SYS_RESET_R {
        REG_CTRL_SYS_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_ctrl_pico_reset(&self) -> REG_CTRL_PICO_RESET_R {
        REG_CTRL_PICO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_ctrl_cpu2_reset(&self) -> REG_CTRL_CPU2_RESET_R {
        REG_CTRL_CPU2_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_ctrl_chip_reset(&self) -> REG_CTRL_CHIP_RESET_R {
        REG_CTRL_CHIP_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_wl_wdt_reset_mm_en(&self) -> REG_WL_WDT_RESET_MM_EN_R {
        REG_WL_WDT_RESET_MM_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_mmwdt2wl_rst_msk(&self) -> REG_MMWDT2WL_RST_MSK_R {
        REG_MMWDT2WL_RST_MSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn reserved_8_23(&self) -> RESERVED_8_23_R {
        RESERVED_8_23_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PKA_CLK_SEL_R {
        PKA_CLK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn reserved_25_27(&self) -> RESERVED_25_27_R {
        RESERVED_25_27_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> REG_CTRL_RESET_DUMMY_R {
        REG_CTRL_RESET_DUMMY_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_pwron_rst(&mut self) -> REG_CTRL_PWRON_RST_W<0> {
        REG_CTRL_PWRON_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_cpu_reset(&mut self) -> REG_CTRL_CPU_RESET_W<1> {
        REG_CTRL_CPU_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_sys_reset(&mut self) -> REG_CTRL_SYS_RESET_W<2> {
        REG_CTRL_SYS_RESET_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_pico_reset(&mut self) -> REG_CTRL_PICO_RESET_W<3> {
        REG_CTRL_PICO_RESET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_cpu2_reset(&mut self) -> REG_CTRL_CPU2_RESET_W<4> {
        REG_CTRL_CPU2_RESET_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_chip_reset(&mut self) -> REG_CTRL_CHIP_RESET_W<5> {
        REG_CTRL_CHIP_RESET_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_wl_wdt_reset_mm_en(&mut self) -> REG_WL_WDT_RESET_MM_EN_W<6> {
        REG_WL_WDT_RESET_MM_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mmwdt2wl_rst_msk(&mut self) -> REG_MMWDT2WL_RST_MSK_W<7> {
        REG_MMWDT2WL_RST_MSK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pka_clk_sel(&mut self) -> PKA_CLK_SEL_W<24> {
        PKA_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_reset_dummy(&mut self) -> REG_CTRL_RESET_DUMMY_W<28> {
        REG_CTRL_RESET_DUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg2](index.html) module"]
pub struct SWRST_CFG2_SPEC;
impl crate::RegisterSpec for SWRST_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg2::R](R) reader structure"]
impl crate::Readable for SWRST_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg2::W](W) writer structure"]
impl crate::Writable for SWRST_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg2 to value 0x90"]
impl crate::Resettable for SWRST_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x90;
}
