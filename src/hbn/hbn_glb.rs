#[doc = "Register `hbn_glb` reader"]
pub struct R(crate::R<HBN_GLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_GLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_GLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_GLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbn_glb` writer"]
pub struct W(crate::W<HBN_GLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_GLB_SPEC>;
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
impl From<crate::W<HBN_GLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_GLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HBN_ROOT_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub type HBN_ROOT_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HBN_UART_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub type HBN_UART_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_GLB_SPEC, bool, O>;
#[doc = "Field `hbn_f32k_sel` reader - "]
pub type HBN_F32K_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_f32k_sel` writer - "]
pub type HBN_F32K_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_5_6` reader - "]
pub type RESERVED_5_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_reset_event` reader - "]
pub type HBN_RESET_EVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_clr_reset_event` reader - "]
pub type HBN_CLR_RESET_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `hbn_clr_reset_event` writer - "]
pub type HBN_CLR_RESET_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_GLB_SPEC, bool, O>;
#[doc = "Field `reserved_14` reader - "]
pub type RESERVED_14_R = crate::BitReader<bool>;
#[doc = "Field `hbn_uart_clk_sel2` reader - "]
pub type HBN_UART_CLK_SEL2_R = crate::BitReader<bool>;
#[doc = "Field `hbn_uart_clk_sel2` writer - "]
pub type HBN_UART_CLK_SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_GLB_SPEC, bool, O>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` reader - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` writer - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `reserved_20_23` reader - "]
pub type RESERVED_20_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11_rt_vout_sel` reader - "]
pub type SW_LDO11_RT_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11_rt_vout_sel` writer - "]
pub type SW_LDO11_RT_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
#[doc = "Field `sw_ldo11_aon_vout_sel` reader - "]
pub type SW_LDO11_AON_VOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sw_ldo11_aon_vout_sel` writer - "]
pub type SW_LDO11_AON_VOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_GLB_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HBN_F32K_SEL_R {
        HBN_F32K_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reserved_5_6(&self) -> RESERVED_5_6_R {
        RESERVED_5_6_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn hbn_reset_event(&self) -> HBN_RESET_EVENT_R {
        HBN_RESET_EVENT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_clr_reset_event(&self) -> HBN_CLR_RESET_EVENT_R {
        HBN_CLR_RESET_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reserved_14(&self) -> RESERVED_14_R {
        RESERVED_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel2(&self) -> HBN_UART_CLK_SEL2_R {
        HBN_UART_CLK_SEL2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SW_LDO11SOC_VOUT_SEL_AON_R {
        SW_LDO11SOC_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reserved_20_23(&self) -> RESERVED_20_23_R {
        RESERVED_20_23_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SW_LDO11_RT_VOUT_SEL_R {
        SW_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SW_LDO11_AON_VOUT_SEL_R {
        SW_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W<0> {
        HBN_ROOT_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_uart_clk_sel(&mut self) -> HBN_UART_CLK_SEL_W<2> {
        HBN_UART_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_f32k_sel(&mut self) -> HBN_F32K_SEL_W<3> {
        HBN_F32K_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_clr_reset_event(&mut self) -> HBN_CLR_RESET_EVENT_W<13> {
        HBN_CLR_RESET_EVENT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_uart_clk_sel2(&mut self) -> HBN_UART_CLK_SEL2_W<15> {
        HBN_UART_CLK_SEL2_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SW_LDO11SOC_VOUT_SEL_AON_W<16> {
        SW_LDO11SOC_VOUT_SEL_AON_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SW_LDO11_RT_VOUT_SEL_W<24> {
        SW_LDO11_RT_VOUT_SEL_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SW_LDO11_AON_VOUT_SEL_W<28> {
        SW_LDO11_AON_VOUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_GLB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_glb](index.html) module"]
pub struct HBN_GLB_SPEC;
impl crate::RegisterSpec for HBN_GLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_glb::R](R) reader structure"]
impl crate::Readable for HBN_GLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_glb::W](W) writer structure"]
impl crate::Writable for HBN_GLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbn_glb to value 0xaa0a_0000"]
impl crate::Resettable for HBN_GLB_SPEC {
    const RESET_VALUE: Self::Ux = 0xaa0a_0000;
}
