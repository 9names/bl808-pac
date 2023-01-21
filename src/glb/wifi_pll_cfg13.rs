#[doc = "Register `wifi_pll_cfg13` reader"]
pub struct R(crate::R<WIFI_PLL_CFG13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_PLL_CFG13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_PLL_CFG13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_PLL_CFG13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `wifi_pll_cfg13` writer"]
pub struct W(crate::W<WIFI_PLL_CFG13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_PLL_CFG13_SPEC>;
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
impl From<crate::W<WIFI_PLL_CFG13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_PLL_CFG13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifipll_resv` reader - "]
pub type WIFIPLL_RESV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wifipll_resv` writer - "]
pub type WIFIPLL_RESV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_PLL_CFG13_SPEC, u16, u16, 16, O>;
#[doc = "Field `reserved_16_20` reader - "]
pub type RESERVED_16_20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usbpll_dl_ctrl` reader - "]
pub type USBPLL_DL_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `usbpll_dl_ctrl` writer - "]
pub type USBPLL_DL_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_30_bz_adc` reader - "]
pub type WIFIPLL_DL_CTRL_30_BZ_ADC_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_30_bz_adc` writer - "]
pub type WIFIPLL_DL_CTRL_30_BZ_ADC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_30` reader - "]
pub type WIFIPLL_DL_CTRL_30_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_30` writer - "]
pub type WIFIPLL_DL_CTRL_30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_20` reader - "]
pub type WIFIPLL_DL_CTRL_20_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_20` writer - "]
pub type WIFIPLL_DL_CTRL_20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_12` reader - "]
pub type WIFIPLL_DL_CTRL_12_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_12` writer - "]
pub type WIFIPLL_DL_CTRL_12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_10` reader - "]
pub type WIFIPLL_DL_CTRL_10_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_10` writer - "]
pub type WIFIPLL_DL_CTRL_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_8` reader - "]
pub type WIFIPLL_DL_CTRL_8_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_8` writer - "]
pub type WIFIPLL_DL_CTRL_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_6` reader - "]
pub type WIFIPLL_DL_CTRL_6_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_6` writer - "]
pub type WIFIPLL_DL_CTRL_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_5` reader - "]
pub type WIFIPLL_DL_CTRL_5_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_5` writer - "]
pub type WIFIPLL_DL_CTRL_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_4` reader - "]
pub type WIFIPLL_DL_CTRL_4_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_4` writer - "]
pub type WIFIPLL_DL_CTRL_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
#[doc = "Field `wifipll_dl_ctrl_2` reader - "]
pub type WIFIPLL_DL_CTRL_2_R = crate::BitReader<bool>;
#[doc = "Field `wifipll_dl_ctrl_2` writer - "]
pub type WIFIPLL_DL_CTRL_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WIFI_PLL_CFG13_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wifipll_resv(&self) -> WIFIPLL_RESV_R {
        WIFIPLL_RESV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reserved_16_20(&self) -> RESERVED_16_20_R {
        RESERVED_16_20_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_dl_ctrl(&self) -> USBPLL_DL_CTRL_R {
        USBPLL_DL_CTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30_bz_adc(&self) -> WIFIPLL_DL_CTRL_30_BZ_ADC_R {
        WIFIPLL_DL_CTRL_30_BZ_ADC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30(&self) -> WIFIPLL_DL_CTRL_30_R {
        WIFIPLL_DL_CTRL_30_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_20(&self) -> WIFIPLL_DL_CTRL_20_R {
        WIFIPLL_DL_CTRL_20_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_12(&self) -> WIFIPLL_DL_CTRL_12_R {
        WIFIPLL_DL_CTRL_12_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_10(&self) -> WIFIPLL_DL_CTRL_10_R {
        WIFIPLL_DL_CTRL_10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_8(&self) -> WIFIPLL_DL_CTRL_8_R {
        WIFIPLL_DL_CTRL_8_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_6(&self) -> WIFIPLL_DL_CTRL_6_R {
        WIFIPLL_DL_CTRL_6_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_5(&self) -> WIFIPLL_DL_CTRL_5_R {
        WIFIPLL_DL_CTRL_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_4(&self) -> WIFIPLL_DL_CTRL_4_R {
        WIFIPLL_DL_CTRL_4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_2(&self) -> WIFIPLL_DL_CTRL_2_R {
        WIFIPLL_DL_CTRL_2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_resv(&mut self) -> WIFIPLL_RESV_W<0> {
        WIFIPLL_RESV_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_dl_ctrl(&mut self) -> USBPLL_DL_CTRL_W<21> {
        USBPLL_DL_CTRL_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_30_bz_adc(&mut self) -> WIFIPLL_DL_CTRL_30_BZ_ADC_W<22> {
        WIFIPLL_DL_CTRL_30_BZ_ADC_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_30(&mut self) -> WIFIPLL_DL_CTRL_30_W<23> {
        WIFIPLL_DL_CTRL_30_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_20(&mut self) -> WIFIPLL_DL_CTRL_20_W<24> {
        WIFIPLL_DL_CTRL_20_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_12(&mut self) -> WIFIPLL_DL_CTRL_12_W<25> {
        WIFIPLL_DL_CTRL_12_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_10(&mut self) -> WIFIPLL_DL_CTRL_10_W<26> {
        WIFIPLL_DL_CTRL_10_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_8(&mut self) -> WIFIPLL_DL_CTRL_8_W<27> {
        WIFIPLL_DL_CTRL_8_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_6(&mut self) -> WIFIPLL_DL_CTRL_6_W<28> {
        WIFIPLL_DL_CTRL_6_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_5(&mut self) -> WIFIPLL_DL_CTRL_5_W<29> {
        WIFIPLL_DL_CTRL_5_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_4(&mut self) -> WIFIPLL_DL_CTRL_4_W<30> {
        WIFIPLL_DL_CTRL_4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_2(&mut self) -> WIFIPLL_DL_CTRL_2_W<31> {
        WIFIPLL_DL_CTRL_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wifi_pll_cfg13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_pll_cfg13](index.html) module"]
pub struct WIFI_PLL_CFG13_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CFG13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_pll_cfg13::R](R) reader structure"]
impl crate::Readable for WIFI_PLL_CFG13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_pll_cfg13::W](W) writer structure"]
impl crate::Writable for WIFI_PLL_CFG13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_cfg13 to value 0"]
impl crate::Resettable for WIFI_PLL_CFG13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
